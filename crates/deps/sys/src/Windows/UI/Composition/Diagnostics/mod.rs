#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompositionDebugHeatMaps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: Self = Self(0u32);
    pub const OffscreenRendered: Self = Self(1u32);
    pub const Colors: Self = Self(2u32);
    pub const Effects: Self = Self(4u32);
    pub const Shadows: Self = Self(8u32);
    pub const Lights: Self = Self(16u32);
    pub const Surfaces: Self = Self(32u32);
    pub const SwapChains: Self = Self(64u32);
    pub const All: Self = Self(4294967295u32);
}
#[repr(transparent)]
pub struct CompositionDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugHeatMaps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugSettingsStatics(pub *mut ::core::ffi::c_void);
