#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWiFiDirectService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiserFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertisementStatus(pub i32);
impl WiFiDirectServiceAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceConfigurationMethod(pub i32);
impl WiFiDirectServiceConfigurationMethod {
    pub const Default: Self = Self(0i32);
    pub const PinDisplay: Self = Self(1i32);
    pub const PinEntry: Self = Self(2i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceError(pub i32);
impl WiFiDirectServiceError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const UnsupportedHardware: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceIPProtocol(pub i32);
impl WiFiDirectServiceIPProtocol {
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionErrorStatus(pub i32);
impl WiFiDirectServiceSessionErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Disassociated: Self = Self(1i32);
    pub const LocalClose: Self = Self(2i32);
    pub const RemoteClose: Self = Self(3i32);
    pub const SystemFailure: Self = Self(4i32);
    pub const NoResponseFromRemote: Self = Self(5i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionStatus(pub i32);
impl WiFiDirectServiceSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Initiated: Self = Self(1i32);
    pub const Requested: Self = Self(2i32);
    pub const Open: Self = Self(3i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceStatus(pub i32);
impl WiFiDirectServiceStatus {
    pub const Available: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
