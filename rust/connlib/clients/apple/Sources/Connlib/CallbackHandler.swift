//
//  CallbackHandler.swift
//

import NetworkExtension
import OSLog

// When the FFI changes from the Rust side, change the CallbackHandler
// functions along with that, but not the delegate protocol.
// When the app gets updated to use the FFI, the delegate protocol
// shall get updated.
// This is so that the app stays buildable even when the FFI changes.

// TODO: https://github.com/chinedufn/swift-bridge/issues/150
extension RustString: @unchecked Sendable {}
extension RustString: Error {}

public protocol CallbackHandlerDelegate: AnyObject {
  func onSetInterfaceConfig(tunnelAddressIPv4: String, tunnelAddressIPv6: String, dnsAddress: String, dnsFallbackStrategy: String)
  func onTunnelReady()
  func onAddRoute(_: String)
  func onRemoveRoute(_: String)
  func onUpdateResources(resourceList: String)
  func onDisconnect(error: Optional<String>)
  func onError(error: String)
}

public class CallbackHandler {
  public weak var delegate: CallbackHandlerDelegate?
  private let logger = Logger.make(for: CallbackHandler.self)

  func onSetInterfaceConfig(tunnelAddressIPv4: RustString, tunnelAddressIPv6: RustString, dnsAddress: RustString, dnsFallbackStrategy: RustString) {
    logger.debug("CallbackHandler.onSetInterfaceConfig: IPv4: \(tunnelAddressIPv4.toString(), privacy: .public), IPv6: \(tunnelAddressIPv6.toString(), privacy: .public), DNS: \(dnsAddress.toString(), privacy: .public)")
    delegate?.onSetInterfaceConfig(
      tunnelAddressIPv4: tunnelAddressIPv4.toString(),
      tunnelAddressIPv6: tunnelAddressIPv6.toString(),
      dnsAddress: dnsAddress.toString(),
      dnsFallbackStrategy: dnsFallbackStrategy.toString()
    )
  }

  func onTunnelReady() {
    logger.debug("CallbackHandler.onTunnelReady")
    delegate?.onTunnelReady()
  }

  func onAddRoute(route: RustString) {
    logger.debug("CallbackHandler.onAddRoute: \(route.toString(), privacy: .public)")
    delegate?.onAddRoute(route.toString())
  }

  func onRemoveRoute(route: RustString) {
    logger.debug("CallbackHandler.onRemoveRoute: \(route.toString(), privacy: .public)")
    delegate?.onRemoveRoute(route.toString())
  }

  func onUpdateResources(resourceList: RustString) {
    logger.debug("CallbackHandler.onUpdateResources: \(resourceList.toString(), privacy: .public)")
    delegate?.onUpdateResources(resourceList: resourceList.toString())
  }

  func onDisconnect(error: RustString) {
    logger.debug("CallbackHandler.onDisconnect: \(error.toString(), privacy: .public)")
    let error = error.toString()
    var optionalError = Optional.some(error)
    if error.isEmpty {
      optionalError = Optional.none
    }
    delegate?.onDisconnect(error: optionalError)
  }

  func onError(error: RustString) {
    logger.debug("CallbackHandler.onError: \(error.toString(), privacy: .public)")
    delegate?.onError(error: error.toString())
  }
}
