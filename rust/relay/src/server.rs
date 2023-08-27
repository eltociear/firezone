mod channel_data;
mod client_message;

pub use crate::server::channel_data::ChannelData;
pub use crate::server::client_message::{
    Allocate, Binding, ChannelBind, ClientMessage, CreatePermission, Refresh,
};

use crate::auth::{MessageIntegrityExt, Nonces, FIREZONE};
use crate::net_ext::IpAddrExt;
use crate::rfc8656::{
    AdditionalAddressFamily, AddressFamily, AddressFamilyNotSupported, PeerAddressFamilyMismatch,
    RequestedAddressFamily,
};
use crate::stun_codec_ext::{MessageClassExt, MethodExt};
use crate::{IpStack, TimeEvents};
use anyhow::Result;
use bytecodec::EncodeExt;
use core::fmt;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, SystemTime};
use stun_codec::rfc5389::attributes::{
    ErrorCode, MessageIntegrity, Nonce, Realm, Username, XorMappedAddress,
};
use stun_codec::rfc5389::errors::{BadRequest, StaleNonce, Unauthorized};
use stun_codec::rfc5389::methods::BINDING;
use stun_codec::rfc5766::attributes::{
    ChannelNumber, Lifetime, RequestedTransport, XorPeerAddress, XorRelayAddress,
};
use stun_codec::rfc5766::errors::{AllocationMismatch, InsufficientCapacity};
use stun_codec::rfc5766::methods::{ALLOCATE, CHANNEL_BIND, CREATE_PERMISSION, REFRESH};
use stun_codec::{Message, MessageClass, MessageEncoder, Method, TransactionId};
use tracing::{field, log};
use uuid::Uuid;

/// A sans-IO STUN & TURN server.
///
/// A [`Server`] is bound to an IPv4 address and assumes to only operate on UDP.
/// Thus, 3 out of the 5 components of a "5-tuple" are unique to an instance of [`Server`] and
/// we can index data simply by the sender's [`SocketAddr`].
///
/// Additionally, we assume to have complete ownership over the port range `lowest_port` - `highest_port`.
pub struct Server<R> {
    decoder: client_message::Decoder,
    encoder: MessageEncoder<Attribute>,

    public_address: IpStack,

    /// All client allocations, indexed by client's socket address.
    allocations: HashMap<SocketAddr, Allocation>,
    clients_by_allocation: HashMap<AllocationId, SocketAddr>,
    allocations_by_port: HashMap<u16, AllocationId>,

    lowest_port: u16,
    highest_port: u16,

    channels_by_number: HashMap<u16, Channel>,
    channel_numbers_by_peer: HashMap<SocketAddr, u16>,

    pending_commands: VecDeque<Command>,
    next_allocation_id: AllocationId,

    rng: R,

    auth_secret: String,

    nonces: Nonces,

    time_events: TimeEvents<TimedAction>,

    allocations_gauge: Gauge,
    responses_counter: Family<ResponsesTotalLabels, Counter>,
    data_relayed_counter: Counter,
}

/// The commands returned from a [`Server`].
///
/// The [`Server`] itself is sans-IO, meaning it is the caller responsibility to cause the side-effects described by these commands.
#[derive(Debug, PartialEq)]
pub enum Command {
    SendMessage {
        payload: Vec<u8>,
        recipient: SocketAddr,
    },
    /// Listen for traffic on the provided port [AddressFamily].
    ///
    /// Any incoming data should be handed to the [`Server`] via [`Server::handle_relay_input`].
    /// A single allocation can reference one of either [AddressFamily]s or both.
    /// Only the combination of [AllocationId] and [AddressFamily] is unique.
    CreateAllocation {
        id: AllocationId,
        family: AddressFamily,
        port: u16,
    },
    /// Free the allocation associated with the given [`AllocationId`] and [AddressFamily]
    FreeAllocation {
        id: AllocationId,
        family: AddressFamily,
    },

    ForwardData {
        id: AllocationId,
        data: Vec<u8>,
        receiver: SocketAddr,
    },
    /// At the latest, the [`Server`] needs to be woken at the specified deadline to execute time-based actions correctly.
    Wake { deadline: SystemTime },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AllocationId(u64);

impl AllocationId {
    fn next(&mut self) -> Self {
        let id = self.0;

        self.0 += 1;

        AllocationId(id)
    }
}

impl fmt::Display for AllocationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AID-{}", self.0)
    }
}

/// See <https://www.rfc-editor.org/rfc/rfc8656#name-requested-transport>.
const UDP_TRANSPORT: u8 = 17;

/// The duration of a channel binding.
///
/// See <https://www.rfc-editor.org/rfc/rfc8656#name-channels-2>.
const CHANNEL_BINDING_DURATION: Duration = Duration::from_secs(600);

impl<R> Server<R>
where
    R: Rng,
{
    /// Constructs a new [`Server`].
    ///
    /// # Port configuration
    ///
    /// The [TURN RFC](https://www.rfc-editor.org/rfc/rfc8656#section-7.2-6) recommends using the port range `49152 - 65535`.
    /// We make this configurable here because there are several situations in which we don't want to use the full range:
    /// - Users might already have other services deployed on the same machine that overlap with the ports the RFC recommends.
    /// - Docker Desktop struggles with forwarding large port ranges to the host with the default networking mode.
    pub fn new(
        public_address: impl Into<IpStack>,
        mut rng: R,
        registry: &mut Registry,
        lowest_port: u16,
        highest_port: u16,
    ) -> Self {
        // TODO: Validate that local IP isn't multicast / loopback etc.

        let allocations_gauge = Gauge::default();
        registry.register(
            "allocations_total",
            "The number of active allocations",
            allocations_gauge.clone(),
        );

        let responses_counter = Family::<ResponsesTotalLabels, Counter>::default();
        registry.register(
            "responses",
            "The number of responses",
            responses_counter.clone(),
        );

        let data_relayed_counter = Counter::default();
        registry.register(
            "data_relayed_bytes",
            "The number of bytes relayed",
            data_relayed_counter.clone(),
        );

        Self {
            decoder: Default::default(),
            encoder: Default::default(),
            public_address: public_address.into(),
            allocations: Default::default(),
            clients_by_allocation: Default::default(),
            allocations_by_port: Default::default(),
            lowest_port,
            highest_port,
            channels_by_number: Default::default(),
            channel_numbers_by_peer: Default::default(),
            pending_commands: Default::default(),
            next_allocation_id: AllocationId(1),
            auth_secret: hex::encode(rng.gen::<[u8; 32]>()),
            rng,
            time_events: TimeEvents::default(),
            nonces: Default::default(),
            allocations_gauge,
            responses_counter,
            data_relayed_counter,
        }
    }

    pub fn auth_secret(&self) -> &str {
        &self.auth_secret
    }

    /// Registers a new, valid nonce.
    ///
    /// Each nonce is valid for 10 requests.
    pub fn add_nonce(&mut self, nonce: Uuid) {
        self.nonces.add_new(nonce);
    }

    /// Process the bytes received from a client.
    ///
    /// After calling this method, you should call [`Server::next_command`] until it returns `None`.
    pub fn handle_client_input(&mut self, bytes: &[u8], sender: SocketAddr, now: SystemTime) {
        let span = tracing::error_span!("client", %sender,  transaction_id = field::Empty);
        let _guard = span.enter();

        if tracing::enabled!(target: "wire", tracing::Level::TRACE) {
            let hex_bytes = hex::encode(bytes);
            tracing::trace!(target: "wire", %hex_bytes, "receiving bytes");
        }

        match self.decoder.decode(bytes) {
            Ok(Ok(message)) => {
                if let Some(id) = message.transaction_id() {
                    span.record("transaction_id", hex::encode(id.as_bytes()));
                }

                self.handle_client_message(message, sender, now);
            }
            // Could parse the bytes but message was semantically invalid (like missing attribute).
            Ok(Err(error_code)) => {
                self.queue_error_response(sender, error_code);
            }
            // Parsing the bytes failed.
            Err(client_message::Error::BadChannelData(ref error)) => {
                tracing::debug!(
                    error = error as &dyn std::error::Error,
                    "failed to decode channel data"
                )
            }
            Err(client_message::Error::DecodeStun(ref error)) => {
                tracing::debug!(
                    error = error as &dyn std::error::Error,
                    "failed to decode stun packet"
                )
            }
            Err(client_message::Error::UnknownMessageType(t)) => {
                tracing::debug!(r#type = %t, "unknown STUN message type")
            }
            Err(client_message::Error::Eof) => {
                tracing::debug!("unexpected EOF while parsing message")
            }
        };
    }

    pub fn handle_client_message(
        &mut self,
        message: ClientMessage,
        sender: SocketAddr,
        now: SystemTime,
    ) {
        let result = match message {
            ClientMessage::Allocate(request) => self.handle_allocate_request(request, sender, now),
            ClientMessage::Refresh(request) => self.handle_refresh_request(request, sender, now),
            ClientMessage::ChannelBind(request) => {
                self.handle_channel_bind_request(request, sender, now)
            }
            ClientMessage::CreatePermission(request) => {
                self.handle_create_permission_request(request, sender, now)
            }
            ClientMessage::Binding(request) => {
                self.handle_binding_request(request, sender);
                return;
            }
            ClientMessage::ChannelData(msg) => {
                self.handle_channel_data_message(msg, sender, now);
                return;
            }
        };

        let Err(error_response) = result else {
            return;
        };

        self.queue_error_response(sender, error_response)
    }

    fn queue_error_response(&mut self, sender: SocketAddr, mut error_response: Message<Attribute>) {
        // In case of a 401 or 438 response, attach a realm and nonce.
        if error_response
            .get_attribute::<ErrorCode>()
            .map_or(false, |error| {
                error == &ErrorCode::from(Unauthorized) || error == &ErrorCode::from(StaleNonce)
            })
        {
            let new_nonce = Uuid::from_u128(self.rng.gen());

            self.add_nonce(new_nonce);

            error_response.add_attribute(Nonce::new(new_nonce.to_string()).unwrap().into());
            error_response.add_attribute((*FIREZONE).clone().into());
        }

        self.send_message(error_response, sender);
    }

    /// Process the bytes received from an allocation.
    pub fn handle_relay_input(
        &mut self,
        bytes: &[u8],
        sender: SocketAddr,
        allocation_id: AllocationId,
    ) {
        let span = tracing::error_span!("peer", %sender, %allocation_id, recipient = field::Empty, channel = field::Empty);
        let _guard = span.enter();

        if tracing::enabled!(target: "wire", tracing::Level::TRACE) {
            let hex_bytes = hex::encode(bytes);
            tracing::trace!(target: "wire", %hex_bytes, "receiving bytes");
        }

        let Some(recipient) = self.clients_by_allocation.get(&allocation_id) else {
            tracing::debug!(target: "relay", "unknown allocation");
            return;
        };

        span.record("recipient", field::display(&recipient));

        let Some(channel_number) = self.channel_numbers_by_peer.get(&sender) else {
            tracing::debug!(target: "relay", "no active channel, refusing to relay {} bytes", bytes.len());
            return;
        };

        span.record("channel", channel_number);

        let Some(channel) = self.channels_by_number.get(channel_number) else {
            debug_assert!(false, "unknown channel {}", channel_number);
            return
        };

        if !channel.bound {
            tracing::debug!(target: "relay", "channel existed but is unbound");
            return;
        }

        if channel.allocation != allocation_id {
            tracing::debug!(target: "relay", "channel is not associated with allocation");
            return;
        }

        tracing::debug!(target: "relay", "Relaying {} bytes", bytes.len());

        self.data_relayed_counter.inc_by(bytes.len() as u64);

        let data = ChannelData::new(*channel_number, bytes).to_bytes();

        if tracing::enabled!(target: "wire", tracing::Level::TRACE) {
            let hex_bytes = hex::encode(&data);
            tracing::trace!(target: "wire", %hex_bytes, "sending bytes");
        }

        self.pending_commands.push_back(Command::SendMessage {
            payload: data,
            recipient: *recipient,
        })
    }

    pub fn handle_deadline_reached(&mut self, now: SystemTime) {
        for action in self.time_events.pending_actions(now) {
            match action {
                TimedAction::ExpireAllocation(id) => {
                    let Some(allocation) = self.get_allocation(&id) else {
                        tracing::debug!(target: "relay", "Cannot expire non-existing allocation {id}");

                        continue;
                    };

                    if allocation.is_expired(now) {
                        self.delete_allocation(id)
                    }
                }
                TimedAction::UnbindChannel(chan) => {
                    let Some(channel) = self.channels_by_number.get_mut(&chan) else {
                        tracing::debug!(target: "relay", "Cannot expire non-existing channel binding {chan}");

                        continue;
                    };

                    if channel.is_expired(now) {
                        tracing::info!(target: "relay", "Channel {chan} is now expired");

                        channel.bound = false;

                        self.time_events.add(
                            now + Duration::from_secs(5 * 60),
                            TimedAction::DeleteChannel(chan),
                        );
                    }
                }
                TimedAction::DeleteChannel(chan) => {
                    self.delete_channel_binding(chan);
                }
            }
        }
    }

    /// An allocation failed.
    pub fn handle_allocation_failed(&mut self, id: AllocationId) {
        self.delete_allocation(id)
    }

    /// Return the next command to be executed.
    pub fn next_command(&mut self) -> Option<Command> {
        self.pending_commands.pop_front()
    }

    fn handle_binding_request(&mut self, message: Binding, sender: SocketAddr) {
        let mut message = Message::new(
            MessageClass::SuccessResponse,
            BINDING,
            message.transaction_id(),
        );
        message.add_attribute(XorMappedAddress::new(sender).into());

        self.send_message(message, sender);
    }

    /// Handle a TURN allocate request.
    ///
    /// See <https://www.rfc-editor.org/rfc/rfc8656#name-receiving-an-allocate-reque> for details.
    fn handle_allocate_request(
        &mut self,
        request: Allocate,
        sender: SocketAddr,
        now: SystemTime,
    ) -> Result<(), Message<Attribute>> {
        let effective_lifetime = request.effective_lifetime();

        let span =
            tracing::error_span!("allocate", lifetime = %effective_lifetime.lifetime().as_secs());
        let _guard = span.enter();

        self.verify_auth(&request, now)?;

        if self.allocations.contains_key(&sender) {
            return Err(error_response(AllocationMismatch, &request));
        }

        if self.allocations_by_port.len() == self.max_available_ports() as usize {
            return Err(error_response(InsufficientCapacity, &request));
        }

        if request.requested_transport().protocol() != UDP_TRANSPORT {
            return Err(error_response(BadRequest, &request));
        }

        let (first_relay_address, maybe_second_relay_addr) = derive_relay_addresses(
            self.public_address,
            request.requested_address_family(),
            request.additional_address_family(),
        )
        .map_err(|e| error_response(e, &request))?;

        // TODO: Do we need to handle DONT-FRAGMENT?
        // TODO: Do we need to handle EVEN/ODD-PORT?

        let allocation = self.create_new_allocation(
            now,
            &effective_lifetime,
            first_relay_address,
            maybe_second_relay_addr,
        );

        let mut message = Message::new(
            MessageClass::SuccessResponse,
            ALLOCATE,
            request.transaction_id(),
        );

        let port = allocation.port;

        message
            .add_attribute(XorRelayAddress::new(SocketAddr::new(first_relay_address, port)).into());
        if let Some(second_relay_address) = maybe_second_relay_addr {
            message.add_attribute(
                XorRelayAddress::new(SocketAddr::new(second_relay_address, port)).into(),
            );
        }

        message.add_attribute(XorMappedAddress::new(sender).into());
        message.add_attribute(effective_lifetime.clone().into());

        let wake_deadline = self.time_events.add(
            allocation.expires_at,
            TimedAction::ExpireAllocation(allocation.id),
        );
        self.pending_commands.push_back(Command::Wake {
            deadline: wake_deadline,
        });
        self.pending_commands.push_back(Command::CreateAllocation {
            id: allocation.id,
            family: first_relay_address.family(),
            port,
        });
        if let Some(second_relay_addr) = maybe_second_relay_addr {
            self.pending_commands.push_back(Command::CreateAllocation {
                id: allocation.id,
                family: second_relay_addr.family(),
                port,
            });
        }
        self.send_message(message, sender);

        if let Some(second_relay_addr) = maybe_second_relay_addr {
            tracing::info!(
                target: "relay",
                first_relay_address = field::display(first_relay_address),
                second_relay_address = field::display(second_relay_addr),
                "Created new allocation",
            )
        } else {
            tracing::info!(
                target: "relay",
                first_relay_address = field::display(first_relay_address),
                "Created new allocation",
            )
        }

        self.clients_by_allocation.insert(allocation.id, sender);
        self.allocations.insert(sender, allocation);
        self.allocations_gauge.inc();

        Ok(())
    }

    /// Handle a TURN refresh request.
    ///
    /// See <https://www.rfc-editor.org/rfc/rfc8656#name-receiving-a-refresh-request> for details.
    fn handle_refresh_request(
        &mut self,
        request: Refresh,
        sender: SocketAddr,
        now: SystemTime,
    ) -> Result<(), Message<Attribute>> {
        let effective_lifetime = request.effective_lifetime();

        let span =
            tracing::error_span!("refresh", lifetime = %effective_lifetime.lifetime().as_secs());
        let _guard = span.enter();

        self.verify_auth(&request, now)?;

        // TODO: Verify that this is the correct error code.
        let allocation = self
            .allocations
            .get_mut(&sender)
            .ok_or(error_response(AllocationMismatch, &request))?;

        if effective_lifetime.lifetime().is_zero() {
            let id = allocation.id;

            self.delete_allocation(id);
            self.send_message(
                refresh_success_response(effective_lifetime, request.transaction_id()),
                sender,
            );

            return Ok(());
        }

        allocation.expires_at = now + effective_lifetime.lifetime();

        tracing::info!(
            target: "relay",
            port = %allocation.port,
            "Refreshed allocation",
        );

        let wake_deadline = self.time_events.add(
            allocation.expires_at,
            TimedAction::ExpireAllocation(allocation.id),
        );
        self.pending_commands.push_back(Command::Wake {
            deadline: wake_deadline,
        });
        self.send_message(
            refresh_success_response(effective_lifetime, request.transaction_id()),
            sender,
        );

        Ok(())
    }

    /// Handle a TURN channel bind request.
    ///
    /// See <https://www.rfc-editor.org/rfc/rfc8656#name-receiving-a-channelbind-req> for details.
    fn handle_channel_bind_request(
        &mut self,
        request: ChannelBind,
        sender: SocketAddr,
        now: SystemTime,
    ) -> Result<(), Message<Attribute>> {
        // Note: `channel_number` is enforced to be in the correct range.
        let requested_channel = request.channel_number().value();
        let peer_address = request.xor_peer_address().address();

        let span = tracing::error_span!("channel_bind", %requested_channel, %peer_address, allocation = field::Empty);
        let _guard = span.enter();

        self.verify_auth(&request, now)?;

        let allocation = self
            .allocations
            .get_mut(&sender)
            .ok_or(error_response(AllocationMismatch, &request))?;

        span.record("allocation", allocation.id.to_string());

        // Check that our allocation can handle the requested peer addr.
        if !allocation.can_relay_to(peer_address) {
            return Err(error_response(PeerAddressFamilyMismatch, &request));
        }

        // Ensure the same address isn't already bound to a different channel.
        if let Some(number) = self.channel_numbers_by_peer.get(&peer_address) {
            if number != &requested_channel {
                return Err(error_response(BadRequest, &request));
            }
        }

        // Ensure the channel is not already bound to a different address.
        if let Some(channel) = self.channels_by_number.get_mut(&requested_channel) {
            if channel.peer_address != peer_address {
                return Err(error_response(BadRequest, &request));
            }

            // Binding requests for existing channels act as a refresh for the binding.

            channel.refresh(now);

            tracing::info!(target: "relay", "Refreshed channel binding");

            self.time_events.add(
                channel.expiry,
                TimedAction::UnbindChannel(requested_channel),
            );
            self.send_message(
                channel_bind_success_response(request.transaction_id()),
                sender,
            );

            return Ok(());
        }

        // Channel binding does not exist yet, create it.

        // TODO: Any additional validations would go here.
        // TODO: Capacity checking would go here.

        let allocation_id = allocation.id;
        self.create_channel_binding(requested_channel, peer_address, allocation_id, now);
        self.send_message(
            channel_bind_success_response(request.transaction_id()),
            sender,
        );

        tracing::info!(target: "relay", "Successfully bound channel");

        Ok(())
    }

    /// Handle a TURN create permission request.
    ///
    /// See <https://www.rfc-editor.org/rfc/rfc8656#name-receiving-a-createpermissio> for details.
    ///
    /// This TURN server implementation does not support relaying data other than through channels.
    /// Thus, creating a permission is a no-op that always succeeds.
    fn handle_create_permission_request(
        &mut self,
        message: CreatePermission,
        sender: SocketAddr,
        now: SystemTime,
    ) -> Result<(), Message<Attribute>> {
        self.verify_auth(&message, now)?;

        self.send_message(
            create_permission_success_response(message.transaction_id()),
            sender,
        );

        Ok(())
    }

    fn handle_channel_data_message(&mut self, message: ChannelData, _: SocketAddr, _: SystemTime) {
        let channel_number = message.channel();
        let data = message.data();

        let span = tracing::error_span!("channel_data", channel = %channel_number, recipient = field::Empty);
        let _guard = span.enter();

        let Some(channel) = self.channels_by_number.get(&channel_number) else {
            tracing::debug!(target: "relay", "Channel does not exist, refusing to forward data");
            return;
        };

        // TODO: Do we need to enforce that only the creator of the channel can relay data?
        // The sender of a UDP packet can be spoofed, so why would we bother?

        if !channel.bound {
            tracing::debug!(target: "relay", "Channel exists but is unbound");
            return;
        }

        let recipient = channel.peer_address;
        span.record("recipient", field::display(&recipient));

        tracing::debug!(target: "relay", "Relaying {} bytes", data.len());

        self.data_relayed_counter.inc_by(data.len() as u64);

        if tracing::enabled!(target: "wire", tracing::Level::TRACE) {
            let hex_bytes = hex::encode(data);
            tracing::trace!(target: "wire", %hex_bytes, "sending bytes");
        }

        self.pending_commands.push_back(Command::ForwardData {
            id: channel.allocation,
            data: data.to_vec(),
            receiver: recipient,
        });
    }

    fn verify_auth(
        &mut self,
        request: &(impl StunRequest + ProtectedRequest),
        now: SystemTime,
    ) -> Result<(), Message<Attribute>> {
        let message_integrity = request
            .message_integrity()
            .map_err(|e| error_response(e, request))?;
        let username = request.username().map_err(|e| error_response(e, request))?;
        let nonce = request
            .nonce()
            .map_err(|e| error_response(e, request))?
            .value()
            .parse::<Uuid>()
            .map_err(|e| {
                log::debug!("failed to parse nonce: {e}");

                error_response(Unauthorized, request)
            })?;

        self.nonces
            .handle_nonce_used(nonce)
            .map_err(|_| error_response(StaleNonce, request))?;

        message_integrity
            .verify(&self.auth_secret, username.name(), now)
            .map_err(|_| error_response(Unauthorized, request))?;

        Ok(())
    }

    fn create_new_allocation(
        &mut self,
        now: SystemTime,
        lifetime: &Lifetime,
        first_relay_addr: IpAddr,
        second_relay_addr: Option<IpAddr>,
    ) -> Allocation {
        // First, find an unused port.

        assert!(
            self.allocations_by_port.len() < self.max_available_ports() as usize,
            "No more ports available; this would loop forever"
        );

        let port = loop {
            let candidate = self.rng.gen_range(self.lowest_port..self.highest_port);

            if !self.allocations_by_port.contains_key(&candidate) {
                break candidate;
            }
        };

        // Second, grab a new allocation ID.
        let id = self.next_allocation_id.next();

        self.allocations_by_port.insert(port, id);

        Allocation {
            id,
            port,
            expires_at: now + lifetime.lifetime(),
            first_relay_addr,
            second_relay_addr,
        }
    }

    fn max_available_ports(&self) -> u16 {
        self.highest_port - self.lowest_port
    }

    fn create_channel_binding(
        &mut self,
        requested_channel: u16,
        peer_address: SocketAddr,
        id: AllocationId,
        now: SystemTime,
    ) {
        self.channels_by_number.insert(
            requested_channel,
            Channel {
                expiry: now + CHANNEL_BINDING_DURATION,
                peer_address,
                allocation: id,
                bound: true,
            },
        );
        self.channel_numbers_by_peer
            .insert(peer_address, requested_channel);
    }

    fn send_message(&mut self, message: Message<Attribute>, recipient: SocketAddr) {
        let method = message.method();
        let class = message.class();
        tracing::trace!(target: "relay",  method = %message.method().as_str(), class = %message.class().as_str(), "Sending message");

        let Ok(bytes) = self.encoder.encode_into_bytes(message) else {
            debug_assert!(false, "Encoding should never fail");
            return;
        };

        if tracing::enabled!(target: "wire", tracing::Level::TRACE) {
            let hex_bytes = hex::encode(&bytes);
            tracing::trace!(target: "wire", %hex_bytes, "sending bytes");
        }

        self.pending_commands.push_back(Command::SendMessage {
            payload: bytes,
            recipient,
        });

        // record metrics
        let response_class = match class {
            MessageClass::SuccessResponse => ResponseClass::Success,
            MessageClass::ErrorResponse => ResponseClass::Error,
            _ => return,
        };
        let message_type = match method {
            BINDING => MessageType::Binding,
            ALLOCATE => MessageType::Allocate,
            REFRESH => MessageType::Refresh,
            CHANNEL_BIND => MessageType::ChannelBind,
            CREATE_PERMISSION => MessageType::CreatePermission,
            _ => return,
        };
        self.responses_counter
            .get_or_create(&ResponsesTotalLabels {
                class: response_class,
                message: message_type,
            })
            .inc();
    }

    fn get_allocation(&self, id: &AllocationId) -> Option<&Allocation> {
        self.clients_by_allocation
            .get(id)
            .and_then(|client| self.allocations.get(client))
    }

    fn delete_allocation(&mut self, id: AllocationId) {
        let Some(client) = self.clients_by_allocation.remove(&id) else {
            return;
        };
        let allocation = self
            .allocations
            .remove(&client)
            .expect("internal state mismatch");

        let port = allocation.port;

        self.allocations_by_port.remove(&port);

        self.allocations_gauge.dec();
        self.pending_commands.push_back(Command::FreeAllocation {
            id,
            family: allocation.first_relay_addr.family(),
        });
        if let Some(second_relay_addr) = allocation.second_relay_addr {
            self.pending_commands.push_back(Command::FreeAllocation {
                id,
                family: second_relay_addr.family(),
            })
        }

        tracing::info!(target: "relay", %port, "Deleted allocation");
    }

    fn delete_channel_binding(&mut self, chan: u16) {
        let Some(channel) = self.channels_by_number.get(&chan) else {
            return;
        };

        let addr = channel.peer_address;

        self.channel_numbers_by_peer.remove(&addr);
        self.channels_by_number.remove(&chan);
    }
}

fn refresh_success_response(
    effective_lifetime: Lifetime,
    transaction_id: TransactionId,
) -> Message<Attribute> {
    let mut message = Message::new(MessageClass::SuccessResponse, REFRESH, transaction_id);
    message.add_attribute(effective_lifetime.into());
    message
}

fn channel_bind_success_response(transaction_id: TransactionId) -> Message<Attribute> {
    Message::new(MessageClass::SuccessResponse, CHANNEL_BIND, transaction_id)
}

fn create_permission_success_response(transaction_id: TransactionId) -> Message<Attribute> {
    Message::new(
        MessageClass::SuccessResponse,
        CREATE_PERMISSION,
        transaction_id,
    )
}

/// Represents an allocation of a client.
struct Allocation {
    id: AllocationId,
    /// Data arriving on this port will be forwarded to the client iff there is an active data channel.
    port: u16,
    expires_at: SystemTime,

    first_relay_addr: IpAddr,
    second_relay_addr: Option<IpAddr>,
}

struct Channel {
    /// When the channel expires.
    expiry: SystemTime,

    /// The address of the peer that the channel is bound to.
    peer_address: SocketAddr,

    /// The allocation this channel belongs to.
    allocation: AllocationId,

    /// Whether the channel is currently bound.
    ///
    /// Channels are active for 10 minutes. During this time, data can be relayed through the channel.
    /// After 10 minutes, the channel is considered unbound.
    ///
    /// To prevent race conditions, we MUST NOT use the same channel number for a different peer and vice versa for another 5 minutes after the channel becomes unbound.
    /// Once it becomes unbound, we simply flip this bool and only completely remove the channel after another 5 minutes.
    ///
    /// With the data structure still existing while the channel is unbound, our existing validations cover the above requirement.
    bound: bool,
}

impl Channel {
    fn refresh(&mut self, now: SystemTime) {
        self.expiry = now + CHANNEL_BINDING_DURATION;
    }

    fn is_expired(&self, now: SystemTime) -> bool {
        self.expiry <= now
    }
}

impl Allocation {
    fn can_relay_to(&self, addr: SocketAddr) -> bool {
        // Currently, we only support IPv4, thus any IPv6 address is invalid.
        addr.is_ipv4()
    }
}

impl Allocation {
    fn is_expired(&self, now: SystemTime) -> bool {
        self.expires_at <= now
    }
}

#[derive(PartialEq)]
enum TimedAction {
    ExpireAllocation(AllocationId),
    UnbindChannel(u16),
    DeleteChannel(u16),
}

fn error_response(
    error_code: impl Into<ErrorCode>,
    request: &impl StunRequest,
) -> Message<Attribute> {
    let mut message = Message::new(
        MessageClass::ErrorResponse,
        request.method(),
        request.transaction_id(),
    );
    message.add_attribute(Attribute::from(error_code.into()));

    message
}

/// Derive the relay address for the client based on the request and the supported IP stack of the relay server.
///
/// By default, a client gets an IPv4 address.
/// They can request an _additional_ IPv6 address or only an IPv6 address.
/// This is handled with two different STUN attributes: [AdditionalAddressFamily] and [RequestedAddressFamily].
///
/// The specification mandates certain checks for how these attributes can be used.
/// In a nutshell, the requirements constrain the use such that there is only one way of doing things.
/// For example, it is disallowed to use [RequestedAddressFamily] for IPv6 and requested and an IPv4 address via [AdditionalAddressFamily].
/// If this is desired, clients should simply use [AdditionalAddressFamily] for IPv6.
fn derive_relay_addresses(
    public_address: IpStack,
    requested_addr_family: Option<&RequestedAddressFamily>,
    additional_addr_family: Option<&AdditionalAddressFamily>,
) -> Result<(IpAddr, Option<IpAddr>), ErrorCode> {
    match (
        public_address,
        requested_addr_family.map(|r| r.address_family()),
        additional_addr_family.map(|a| a.address_family()),
    ) {
        (
            IpStack::Ip4(addr) | IpStack::Dual { ip4: addr, .. },
            None | Some(AddressFamily::V4),
            None,
        ) => Ok((addr.into(), None)),
        (IpStack::Ip6(addr) | IpStack::Dual { ip6: addr, .. }, Some(AddressFamily::V6), None) => {
            Ok((addr.into(), None))
        }
        (IpStack::Dual { ip4, ip6 }, None, Some(AddressFamily::V6)) => {
            Ok((ip4.into(), Some(ip6.into())))
        }
        (_, Some(_), Some(_)) => Err(BadRequest.into()),
        (_, _, Some(AddressFamily::V4)) => Err(BadRequest.into()),
        _ => Err(AddressFamilyNotSupported.into()),
    }
}

/// Private helper trait to make [`error_response`] more ergonomic to use.
trait StunRequest {
    fn transaction_id(&self) -> TransactionId;
    fn method(&self) -> Method;
}

macro_rules! impl_stun_request_for {
    ($t:ty, $m:expr) => {
        impl StunRequest for $t {
            fn transaction_id(&self) -> TransactionId {
                self.transaction_id()
            }

            fn method(&self) -> Method {
                $m
            }
        }
    };
}

impl_stun_request_for!(Allocate, ALLOCATE);
impl_stun_request_for!(ChannelBind, CHANNEL_BIND);
impl_stun_request_for!(CreatePermission, CREATE_PERMISSION);
impl_stun_request_for!(Refresh, REFRESH);

/// Private helper trait to make [`Server::verify_auth`] more ergonomic to use.
trait ProtectedRequest {
    fn message_integrity(&self) -> Result<&MessageIntegrity, Unauthorized>;
    fn username(&self) -> Result<&Username, Unauthorized>;
    fn nonce(&self) -> Result<&Nonce, Unauthorized>;
}

macro_rules! impl_protected_request_for {
    ($t:ty) => {
        impl ProtectedRequest for $t {
            fn message_integrity(&self) -> Result<&MessageIntegrity, Unauthorized> {
                self.message_integrity().ok_or(Unauthorized)
            }

            fn username(&self) -> Result<&Username, Unauthorized> {
                self.username().ok_or(Unauthorized)
            }

            fn nonce(&self) -> Result<&Nonce, Unauthorized> {
                self.nonce().ok_or(Unauthorized)
            }
        }
    };
}

impl_protected_request_for!(Allocate);
impl_protected_request_for!(ChannelBind);
impl_protected_request_for!(CreatePermission);
impl_protected_request_for!(Refresh);

// Define an enum of all attributes that we care about for our server.
stun_codec::define_attribute_enums!(
    Attribute,
    AttributeDecoder,
    AttributeEncoder,
    [
        MessageIntegrity,
        XorMappedAddress,
        ErrorCode,
        RequestedTransport,
        XorRelayAddress,
        Lifetime,
        ChannelNumber,
        XorPeerAddress,
        Nonce,
        Realm,
        Username,
        RequestedAddressFamily,
        AdditionalAddressFamily
    ]
);

#[derive(Clone, Debug, Hash, PartialEq, Eq, prometheus_client::encoding::EncodeLabelSet)]
struct ResponsesTotalLabels {
    class: ResponseClass,
    message: MessageType,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, prometheus_client::encoding::EncodeLabelValue)]
enum ResponseClass {
    Success,
    Error,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, prometheus_client::encoding::EncodeLabelValue)]
enum MessageType {
    Binding,
    Allocate,
    ChannelBind,
    CreatePermission,
    Refresh,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    // Tests for requirements listed in https://www.rfc-editor.org/rfc/rfc8656#name-receiving-an-allocate-reque.

    // 6. The server checks if the request contains both REQUESTED-ADDRESS-FAMILY and ADDITIONAL-ADDRESS-FAMILY attributes. If yes, then the server rejects the request with a 400 (Bad Request) error.
    #[test]
    fn requested_and_additional_is_bad_request() {
        let error_code = derive_relay_addresses(
            IpStack::Ip4(Ipv4Addr::LOCALHOST),
            Some(&RequestedAddressFamily::new(AddressFamily::V4)),
            Some(&AdditionalAddressFamily::new(AddressFamily::V6)),
        )
        .unwrap_err();

        assert_eq!(error_code.code(), BadRequest::CODEPOINT)
    }

    // 7. If the server does not support the address family requested by the client in REQUESTED-ADDRESS-FAMILY, or if the allocation of the requested address family is disabled by local policy, it MUST generate an Allocate error response, and it MUST include an ERROR-CODE attribute with the 440 (Address Family not Supported) response code.
    // If the REQUESTED-ADDRESS-FAMILY attribute is absent and the server does not support the IPv4 address family, the server MUST include an ERROR-CODE attribute with the 440 (Address Family not Supported) response code.
    #[test]
    fn requested_address_family_not_available_is_not_supported() {
        let error_code = derive_relay_addresses(
            IpStack::Ip4(Ipv4Addr::LOCALHOST),
            Some(&RequestedAddressFamily::new(AddressFamily::V6)),
            None,
        )
        .unwrap_err();

        assert_eq!(error_code.code(), AddressFamilyNotSupported::CODEPOINT);

        let error_code = derive_relay_addresses(
            IpStack::Ip6(Ipv6Addr::LOCALHOST),
            Some(&RequestedAddressFamily::new(AddressFamily::V4)),
            None,
        )
        .unwrap_err();

        assert_eq!(error_code.code(), AddressFamilyNotSupported::CODEPOINT);

        let error_code =
            derive_relay_addresses(IpStack::Ip6(Ipv6Addr::LOCALHOST), None, None).unwrap_err();

        assert_eq!(error_code.code(), AddressFamilyNotSupported::CODEPOINT)
    }

    //9. The server checks if the request contains an ADDITIONAL-ADDRESS-FAMILY attribute. If yes, and the attribute value is 0x01 (IPv4 address family), then the server rejects the request with a 400 (Bad Request) error.
    #[test]
    fn additional_address_family_ip4_is_bad_request() {
        let error_code = derive_relay_addresses(
            IpStack::Ip4(Ipv4Addr::LOCALHOST),
            None,
            Some(&AdditionalAddressFamily::new(AddressFamily::V4)),
        )
        .unwrap_err();

        assert_eq!(error_code.code(), BadRequest::CODEPOINT)
    }
}
