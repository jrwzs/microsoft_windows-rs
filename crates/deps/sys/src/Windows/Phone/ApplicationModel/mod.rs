#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: Self = Self(0u32);
    pub const Alternate: Self = Self(1u32);
}
#[repr(transparent)]
pub struct IApplicationProfileStatics(pub *mut ::core::ffi::c_void);
