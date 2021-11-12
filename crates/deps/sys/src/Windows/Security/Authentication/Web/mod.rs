#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TokenBindingKeyType(pub i32);
impl TokenBindingKeyType {
    pub const Rsa2048: Self = Self(0i32);
    pub const EcdsaP256: Self = Self(1i32);
    pub const AnyExisting: Self = Self(2i32);
}
#[repr(transparent)]
pub struct WebAuthenticationOptions(pub u32);
impl WebAuthenticationOptions {
    pub const None: Self = Self(0u32);
    pub const SilentMode: Self = Self(1u32);
    pub const UseTitle: Self = Self(2u32);
    pub const UseHttpPost: Self = Self(4u32);
    pub const UseCorporateNetwork: Self = Self(8u32);
}
#[repr(transparent)]
pub struct WebAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAuthenticationStatus(pub i32);
impl WebAuthenticationStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const ErrorHttp: Self = Self(2i32);
}
