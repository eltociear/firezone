//! A resource table is a custom type that allows us to store a resource under an id and possibly multiple ips or even network ranges
use std::{collections::HashMap, net::IpAddr, ptr::NonNull};

use chrono::{DateTime, Utc};
use ip_network::IpNetwork;
use ip_network_table::IpNetworkTable;
use libs_common::messages::{Id, ResourceDescription};

pub(crate) trait Resource {
    fn description(&self) -> &ResourceDescription;
}

impl Resource for ResourceDescription {
    fn description(&self) -> &ResourceDescription {
        self
    }
}

impl Resource for (ResourceDescription, DateTime<Utc>) {
    fn description(&self) -> &ResourceDescription {
        &self.0
    }
}

// Oh boy... here we go
/// The resource table type
///
/// This is specifically crafted for our use case, so the API is particularly made for us and not generic
pub(crate) struct ResourceTable<T> {
    id_table: HashMap<Id, T>,
    network_table: IpNetworkTable<NonNull<T>>,
    dns_name: HashMap<String, NonNull<T>>,
}

// SAFETY: We actually hold a hashmap internally that the pointers points to
unsafe impl<T> Send for ResourceTable<T> {}
// SAFETY: we don't allow interior mutability of the pointers we hold, in fact we don't allow ANY mutability!
// (this is part of the reason why the API is so limiting, it is easier to reason about.
unsafe impl<T> Sync for ResourceTable<T> {}

impl<T> Default for ResourceTable<T> {
    fn default() -> ResourceTable<T> {
        ResourceTable::new()
    }
}

impl<T> ResourceTable<T> {
    /// Creates a new `ResourceTable`
    pub fn new() -> ResourceTable<T> {
        ResourceTable {
            network_table: IpNetworkTable::new(),
            id_table: HashMap::new(),
            dns_name: HashMap::new(),
        }
    }
}

impl<T> ResourceTable<T>
where
    T: Resource + Clone,
{
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.id_table.values()
    }

    pub fn network_resources(&self) -> HashMap<IpNetwork, T> {
        // Safety: Due to internal consistency, since the value is stored the reference should be valid
        self.network_table
            .iter()
            .map(|(wg_ip, res)| (wg_ip, unsafe { res.as_ref() }.clone()))
            .collect()
    }

    pub fn dns_resources(&self) -> HashMap<String, T> {
        // Safety: Due to internal consistency, since the value is stored the reference should be valid
        self.dns_name
            .iter()
            .map(|(name, res)| (name.clone(), unsafe { res.as_ref() }.clone()))
            .collect()
    }

    /// Tells you if it's empty
    pub fn is_empty(&self) -> bool {
        self.id_table.is_empty()
    }

    /// Gets the resource by ip
    pub fn get_by_ip(&self, ip: impl Into<IpAddr>) -> Option<&T> {
        // SAFETY: if we found the pointer, due to our internal consistency rules it is in the id_table
        self.network_table
            .longest_match(ip)
            .map(|m| unsafe { m.1.as_ref() })
    }

    /// Gets the resource by id
    pub fn get_by_id(&self, id: &Id) -> Option<&T> {
        self.id_table.get(id)
    }

    /// Gets the resource by name
    pub fn get_by_name(&self, name: impl AsRef<str>) -> Option<&T> {
        // SAFETY: if we found the pointer, due to our internal consistency rules it is in the id_table
        self.dns_name
            .get(name.as_ref())
            .map(|m| unsafe { m.as_ref() })
    }

    // SAFETY: resource_description must still be in storage since we are going to reference it.
    unsafe fn remove_resource(&mut self, resource_description: NonNull<T>) {
        let id = {
            let res = resource_description.as_ref();
            match res.description() {
                ResourceDescription::Dns(r) => {
                    self.dns_name.remove(&r.address);
                    self.network_table.remove(r.ipv4);
                    self.network_table.remove(r.ipv6);
                    r.id
                }
                ResourceDescription::Cidr(r) => {
                    self.network_table.remove(r.address);
                    r.id
                }
            }
        };
        self.id_table.remove(&id);
    }

    pub(crate) fn cleanup_resource(&mut self, resource_description: &T) {
        match resource_description.description() {
            ResourceDescription::Dns(r) => {
                if let Some(res) = self.id_table.get(&r.id) {
                    // SAFETY: We are consistent that if the item exists on any of the containers it still exists in the storage
                    unsafe {
                        self.remove_resource(res.into());
                    }
                    // Don't use res after here
                }

                if let Some(res) = self.dns_name.remove(&r.address) {
                    // SAFETY: We are consistent that if the item exists on any of the containers it still exists in the storage
                    unsafe {
                        self.remove_resource(res);
                    }
                    // Don't use res after here
                }

                if let Some(res) = self.network_table.remove(r.ipv4) {
                    // SAFETY: We are consistent that if the item exists on any of the containers it still exists in the storage
                    unsafe {
                        self.remove_resource(res);
                    }
                }

                if let Some(res) = self.network_table.remove(r.ipv6) {
                    // SAFETY: We are consistent that if the item exists on any of the containers it still exists in the storage
                    unsafe {
                        self.remove_resource(res);
                    }
                }
            }
            ResourceDescription::Cidr(r) => {
                if let Some(res) = self.id_table.get(&r.id) {
                    // SAFETY: We are consistent that if the item exists on any of the containers it still exists in the storage
                    unsafe {
                        self.remove_resource(res.into());
                    }
                    // Don't use res after here
                }

                if let Some(res) = self.network_table.remove(r.address) {
                    // SAFETY: We are consistent that if the item exists on any of the containers it still exists in the storage
                    unsafe {
                        self.remove_resource(res);
                    }
                }
            }
        }
    }

    // For soundness it's very important that this API only takes a resource_description
    // doing this, we can assume that when removing a resource from the id table we have all the info
    // about all the tables.
    /// Inserts a new resource_description
    ///
    /// If the id was used previously the old value will be deleted.
    /// Same goes if any of the ip matches exactly an old ip or dns name.
    /// This means that a match in IP or dns name will discard all old values.
    ///
    /// This is done so that we don't have dangling values.
    pub fn insert(&mut self, resource_description: T) {
        self.cleanup_resource(&resource_description);
        let id = resource_description.description().id();
        self.id_table.insert(id, resource_description);
        // we just inserted it we can unwrap
        let res = self.id_table.get(&id).unwrap();
        match res.description() {
            ResourceDescription::Dns(r) => {
                self.network_table.insert(r.ipv4, res.into());
                self.network_table.insert(r.ipv6, res.into());
                self.dns_name.insert(r.address.clone(), res.into());
            }
            ResourceDescription::Cidr(r) => {
                self.network_table.insert(r.address, res.into());
            }
        }
    }

    pub fn resource_list(&self) -> Vec<ResourceDescription> {
        self.id_table
            .values()
            .map(|r| r.description())
            .cloned()
            .collect()
    }
}
