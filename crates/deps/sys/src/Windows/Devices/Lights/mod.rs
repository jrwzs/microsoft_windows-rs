#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Lamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayKind(pub i32);
impl LampArrayKind {
    pub const Undefined: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
    pub const GameController: Self = Self(3i32);
    pub const Peripheral: Self = Self(4i32);
    pub const Scene: Self = Self(5i32);
    pub const Notification: Self = Self(6i32);
    pub const Chassis: Self = Self(7i32);
    pub const Wearable: Self = Self(8i32);
    pub const Furniture: Self = Self(9i32);
    pub const Art: Self = Self(10i32);
}
#[repr(transparent)]
pub struct LampAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampPurposes(pub u32);
impl LampPurposes {
    pub const Undefined: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Accent: Self = Self(2u32);
    pub const Branding: Self = Self(4u32);
    pub const Status: Self = Self(8u32);
    pub const Illumination: Self = Self(16u32);
    pub const Presentation: Self = Self(32u32);
}
