//  Adapter.swift
//  (c) 2023 Firezone, Inc.
//  LICENSE: Apache-2.0
//
import FirezoneKit
import Foundation
import NetworkExtension
import OSLog

#if os(iOS)
  import UIKit.UIDevice
#endif

public enum AdapterError: Error {
  /// Failure to perform an operation in such state.
  case invalidState

  /// connlib failed to start
  case connlibConnectError(Error)

  /// connlib fatal error
  case connlibFatalError(String)

  /// No network settings were provided
  case noNetworkSettings

  /// Failure to set network settings.
  case setNetworkSettings(Error)

  /// stop() called before the tunnel became ready
  case stoppedByRequestWhileStarting
}

/// Enum representing internal state of the  adapter
private enum AdapterState: CustomStringConvertible {
  case startingTunnel(session: WrappedSession, onStarted: Adapter.StartTunnelCompletionHandler?)
  case tunnelReady(session: WrappedSession)
  case stoppingTunnel(session: WrappedSession, onStopped: Adapter.StopTunnelCompletionHandler?)
  case stoppedTunnel
  case stoppingTunnelTemporarily(
    session: WrappedSession, onStopped: Adapter.StopTunnelCompletionHandler?)
  case stoppedTunnelTemporarily

  var description: String {
    switch self {
    case .startingTunnel: return "startingTunnel"
    case .tunnelReady: return "tunnelReady"
    case .stoppingTunnel: return "stoppingTunnel"
    case .stoppedTunnel: return "stoppedTunnel"
    case .stoppingTunnelTemporarily: return "stoppingTunnelTemporarily"
    case .stoppedTunnelTemporarily: return "stoppedTunnelTemporarily"
    }
  }
}

// Loosely inspired from WireGuardAdapter from WireGuardKit
public class Adapter {

  typealias StartTunnelCompletionHandler = ((AdapterError?) -> Void)
  typealias StopTunnelCompletionHandler = (() -> Void)

  private let logger = Logger.make(category: "packet-tunnel")

  private var callbackHandler: CallbackHandler

  /// Network settings
  private var networkSettings: NetworkSettings?

  /// Packet tunnel provider.
  private weak var packetTunnelProvider: NEPacketTunnelProvider?

  /// Network routes monitor.
  private var networkMonitor: NWPathMonitor?

  /// Private queue used to synchronize access to `WireGuardAdapter` members.
  private let workQueue = DispatchQueue(label: "FirezoneAdapterWorkQueue")

  /// Adapter state.
  private var state: AdapterState {
    didSet {
      logger.debug("Adapter state changed to: \(self.state, privacy: .public)")
    }
  }

  /// Keep track of resources
  private var displayableResources = DisplayableResources()

  /// Starting parameters
  private var controlPlaneURLString: String
  private var token: String

  public init(
    controlPlaneURLString: String, token: String, packetTunnelProvider: NEPacketTunnelProvider
  ) {
    self.controlPlaneURLString = controlPlaneURLString
    self.token = token
    self.packetTunnelProvider = packetTunnelProvider
    self.callbackHandler = CallbackHandler()
    self.state = .stoppedTunnel
  }

  deinit {
    self.logger.debug("Adapter.deinit")
    // Cancel network monitor
    networkMonitor?.cancel()

    // Shutdown the tunnel
    if case .tunnelReady(let wrappedSession) = self.state {
      logger.debug("Adapter.deinit: Shutting down connlib")
      wrappedSession.disconnect()
    }
  }

  /// Start the tunnel tunnel.
  /// - Parameters:
  ///   - completionHandler: completion handler.
  public func start(completionHandler: @escaping (AdapterError?) -> Void) throws {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.start")
      guard case .stoppedTunnel = self.state else {
        completionHandler(.invalidState)
        return
      }

      self.callbackHandler.delegate = self

      self.logger.debug("Adapter.start: Starting connlib")
      do {
        self.state = .startingTunnel(
          session: try WrappedSession.connect(
            self.controlPlaneURLString, self.token, self.getExternalId(), self.callbackHandler),
          onStarted: completionHandler
        )
      } catch let error {
        self.logger.error("Adapter.start: Error: \(error, privacy: .public)")
        self.state = .stoppedTunnel
        completionHandler(AdapterError.connlibConnectError(error))
      }

    }
  }

  /// Stop the tunnel
  public func stop(completionHandler: @escaping () -> Void) {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.stop")

      switch self.state {
      case .stoppedTunnel, .stoppingTunnel:
        break
      case .tunnelReady(let session):
        self.logger.debug("Adapter.stop: Shutting down connlib")
        self.state = .stoppingTunnel(session: session, onStopped: completionHandler)
        session.disconnect()
      case .startingTunnel(let session, let onStarted):
        self.logger.debug("Adapter.stop: Shutting down connlib before tunnel ready")
        self.state = .stoppingTunnel(
          session: session,
          onStopped: {
            onStarted?(AdapterError.stoppedByRequestWhileStarting)
            completionHandler()
          })
        session.disconnect()
      case .stoppingTunnelTemporarily(let session, let onStopped):
        self.state = .stoppingTunnel(
          session: session,
          onStopped: {
            onStopped?()
            completionHandler()
          })
      case .stoppedTunnelTemporarily:
        self.state = .stoppedTunnel
        completionHandler()
      }

      self.networkMonitor?.cancel()
      self.networkMonitor = nil
    }
  }

  /// Get the current set of resources in the completionHandler.
  /// If unchanged since referenceVersionString, call completionHandler(nil).
  public func getDisplayableResourcesIfVersionDifferentFrom(
    referenceVersionString: String, completionHandler: @escaping (DisplayableResources?) -> Void
  ) {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      if referenceVersionString == self.displayableResources.versionString {
        completionHandler(nil)
      } else {
        completionHandler(self.displayableResources)
      }
    }
  }
}

// MARK: Device unique identifiers
extension Adapter {
  func getExternalId() -> String {
    #if os(iOS)
      guard let uuid = UIDevice.current.identifierForVendor?.uuidString else {
        // Send a blank string, letting either connlib or the portal handle this
        return ""
      }
      return uuid
    #elseif os(macOS)
      guard let macBytes = PrimaryMacAddress.copy_mac_address() else {
        // Send a blank string, letting either connlib or the portal handle this
        return ""
      }
      return (macBytes as Data).base64EncodedString()
    #else
      #error("Unsupported platform")
    #endif
  }
}

// MARK: Responding to path updates

extension Adapter {
  private func beginPathMonitoring() {
    self.logger.debug("Beginning path monitoring")
    let networkMonitor = NWPathMonitor()
    networkMonitor.pathUpdateHandler = { [weak self] path in
      self?.didReceivePathUpdate(path: path)
    }
    networkMonitor.start(queue: self.workQueue)
  }

  private func didReceivePathUpdate(path: Network.NWPath) {
    // Will be invoked in the workQueue by the path monitor
    switch self.state {

    case .startingTunnel(let session, let onStarted):
      if path.status != .satisfied {
        self.logger.debug("Adapter.didReceivePathUpdate: Offline. Shutting down connlib.")
        onStarted?(nil)
        self.packetTunnelProvider?.reasserting = true
        self.state = .stoppingTunnelTemporarily(session: session, onStopped: nil)
        session.disconnect()
      }

    case .tunnelReady(let session):
      if path.status == .satisfied {
        self.logger.debug(
          "Suppressing calls to disableSomeRoamingForBrokenMobileSemantics() and bumpSockets()")
        // #if os(iOS)
        // wrappedSession.disableSomeRoamingForBrokenMobileSemantics()
        // #endif
        // wrappedSession.bumpSockets()
      } else {
        self.logger.debug("Adapter.didReceivePathUpdate: Offline. Shutting down connlib.")
        self.packetTunnelProvider?.reasserting = true
        self.state = .stoppingTunnelTemporarily(session: session, onStopped: nil)
        session.disconnect()
      }

    case .stoppingTunnelTemporarily:
      break

    case .stoppedTunnelTemporarily:
      guard path.status == .satisfied else { return }

      self.logger.debug("Adapter.didReceivePathUpdate: Back online. Starting connlib.")

      do {
        self.state = .startingTunnel(
          session: try WrappedSession.connect(
            controlPlaneURLString, token, self.getExternalId(), self.callbackHandler),
          onStarted: { error in
            if let error = error {
              self.logger.error(
                "Adapter.didReceivePathUpdate: Error starting connlib: \(error, privacy: .public)")
              self.packetTunnelProvider?.cancelTunnelWithError(error)
            } else {
              self.packetTunnelProvider?.reasserting = false
            }
          }
        )
      } catch let error as AdapterError {
        self.logger.error("Adapter.didReceivePathUpdate: Error: \(error, privacy: .public)")
      } catch {
        self.logger.error(
          "Adapter.didReceivePathUpdate: Unknown error: \(error, privacy: .public) (fatal)")
      }

    case .stoppingTunnel, .stoppedTunnel:
      // no-op
      break
    }
  }
}

// MARK: Implementing CallbackHandlerDelegate

extension Adapter: CallbackHandlerDelegate {
  public func onSetInterfaceConfig(
    tunnelAddressIPv4: String, tunnelAddressIPv6: String, dnsAddress: String,
    dnsFallbackStrategy: String
  ) {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.onSetInterfaceConfig")

      switch self.state {
      case .startingTunnel:
        self.networkSettings = NetworkSettings(
          tunnelAddressIPv4: tunnelAddressIPv4, tunnelAddressIPv6: tunnelAddressIPv6,
          dnsAddress: dnsAddress,
          dnsFallbackStrategy: NetworkSettings.DNSFallbackStrategy(dnsFallbackStrategy))
      case .tunnelReady:
        if let networkSettings = self.networkSettings {
          networkSettings.setDNSFallbackStrategy(
            NetworkSettings.DNSFallbackStrategy(dnsFallbackStrategy))
          if let packetTunnelProvider = self.packetTunnelProvider {
            networkSettings.apply(
              on: packetTunnelProvider, logger: self.logger, completionHandler: nil)
          }
        }

      case .stoppingTunnel, .stoppedTunnel, .stoppingTunnelTemporarily, .stoppedTunnelTemporarily:
        // This is not expected to happen
        break
      }
    }
  }

  public func onTunnelReady() {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.onTunnelReady")
      guard case .startingTunnel(let session, let onStarted) = self.state else {
        self.logger.error(
          "Adapter.onTunnelReady: Unexpected state: \(self.state, privacy: .public)")
        return
      }
      guard let networkSettings = self.networkSettings else {
        self.logger.error("Adapter.onTunnelReady: No network settings")
        return
      }
      guard let packetTunnelProvider = self.packetTunnelProvider else {
        self.logger.error("Adapter.onTunnelReady: No packet tunnel provider")
        return
      }
      networkSettings.apply(on: packetTunnelProvider, logger: self.logger) { error in
        if let error = error {
          onStarted?(AdapterError.setNetworkSettings(error))
          self.state = .stoppedTunnel
        } else {
          onStarted?(nil)
          self.state = .tunnelReady(session: session)
          self.beginPathMonitoring()
        }
      }
    }
  }

  public func onAddRoute(_ route: String) {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.onAddRoute(\(route, privacy: .public))")
      guard let networkSettings = self.networkSettings else {
        self.logger.error("Adapter.onAddRoute: No network settings")
        return
      }
      guard let packetTunnelProvider = self.packetTunnelProvider else {
        self.logger.error("Adapter.onAddRoute: No packet tunnel provider")
        return
      }

      networkSettings.addRoute(route)
      if case .tunnelReady = self.state {
        networkSettings.apply(on: packetTunnelProvider, logger: self.logger, completionHandler: nil)
      }
    }
  }

  public func onRemoveRoute(_ route: String) {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.onRemoveRoute(\(route, privacy: .public))")
      guard let networkSettings = self.networkSettings else {
        self.logger.error("Adapter.onRemoveRoute: No network settings")
        return
      }
      guard let packetTunnelProvider = self.packetTunnelProvider else {
        self.logger.error("Adapter.onRemoveRoute: No packet tunnel provider")
        return
      }
      networkSettings.removeRoute(route)
      if case .tunnelReady = self.state {
        networkSettings.apply(on: packetTunnelProvider, logger: self.logger, completionHandler: nil)
      }
    }
  }

  public func onUpdateResources(resourceList: String) {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.onUpdateResources")
      let jsonString = resourceList
      guard let jsonData = jsonString.data(using: .utf8) else {
        return
      }
      guard let networkResources = try? JSONDecoder().decode([NetworkResource].self, from: jsonData)
      else {
        return
      }

      // Note down the resources
      self.displayableResources.update(resources: networkResources.map { $0.displayableResource })

      // Update DNS in case resource domains is changing
      guard let networkSettings = self.networkSettings else {
        self.logger.error("Adapter.onUpdateResources: No network settings")
        return
      }
      guard let packetTunnelProvider = self.packetTunnelProvider else {
        self.logger.error("Adapter.onUpdateResources: No packet tunnel provider")
        return
      }
      let updatedResourceDomains = networkResources.compactMap { $0.resourceLocation.domain }
      networkSettings.setResourceDomains(updatedResourceDomains)
      if case .tunnelReady = self.state {
        networkSettings.apply(on: packetTunnelProvider, logger: self.logger, completionHandler: nil)
      }
    }
  }

  public func onDisconnect(error: String?) {
    workQueue.async { [weak self] in
      guard let self = self else { return }

      self.logger.debug("Adapter.onDisconnect")
      if let errorMessage = error {
        self.logger.error(
          "Connlib disconnected with unrecoverable error: \(errorMessage, privacy: .public)")
        switch self.state {
        case .stoppingTunnel(session: _, let onStopped):
          onStopped?()
          self.state = .stoppedTunnel
        case .stoppingTunnelTemporarily(session: _, let onStopped):
          onStopped?()
          self.state = .stoppedTunnel
        case .stoppedTunnel:
          // This should not happen
          break
        case .stoppedTunnelTemporarily:
          self.state = .stoppedTunnel
        default:
          self.packetTunnelProvider?.cancelTunnelWithError(
            AdapterError.connlibFatalError(errorMessage))
          self.state = .stoppedTunnel
        }
      } else {
        self.logger.debug("Connlib disconnected")
        switch self.state {
        case .stoppingTunnel(session: _, let onStopped):
          onStopped?()
          self.state = .stoppedTunnel
        case .stoppingTunnelTemporarily(session: _, let onStopped):
          onStopped?()
          self.state = .stoppedTunnelTemporarily
        default:
          // This should not happen
          self.state = .stoppedTunnel
        }
      }
    }
  }

  public func onError(error: String) {
    self.logger.error("Internal connlib error: \(error, privacy: .public)")
  }
}
