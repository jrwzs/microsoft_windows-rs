#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EyesPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandJointKind(pub i32);
impl HandJointKind {
    pub const Palm: Self = Self(0i32);
    pub const Wrist: Self = Self(1i32);
    pub const ThumbMetacarpal: Self = Self(2i32);
    pub const ThumbProximal: Self = Self(3i32);
    pub const ThumbDistal: Self = Self(4i32);
    pub const ThumbTip: Self = Self(5i32);
    pub const IndexMetacarpal: Self = Self(6i32);
    pub const IndexProximal: Self = Self(7i32);
    pub const IndexIntermediate: Self = Self(8i32);
    pub const IndexDistal: Self = Self(9i32);
    pub const IndexTip: Self = Self(10i32);
    pub const MiddleMetacarpal: Self = Self(11i32);
    pub const MiddleProximal: Self = Self(12i32);
    pub const MiddleIntermediate: Self = Self(13i32);
    pub const MiddleDistal: Self = Self(14i32);
    pub const MiddleTip: Self = Self(15i32);
    pub const RingMetacarpal: Self = Self(16i32);
    pub const RingProximal: Self = Self(17i32);
    pub const RingIntermediate: Self = Self(18i32);
    pub const RingDistal: Self = Self(19i32);
    pub const RingTip: Self = Self(20i32);
    pub const LittleMetacarpal: Self = Self(21i32);
    pub const LittleProximal: Self = Self(22i32);
    pub const LittleIntermediate: Self = Self(23i32);
    pub const LittleDistal: Self = Self(24i32);
    pub const LittleTip: Self = Self(25i32);
}
#[repr(transparent)]
pub struct HandMeshObserver(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct HandMeshVertex(i32);
#[repr(transparent)]
pub struct HandMeshVertexState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HeadPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEyesPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEyesPoseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandMeshObserver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandMeshVertexState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHeadPose(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct JointPose(i32);
#[repr(transparent)]
pub struct JointPoseAccuracy(pub i32);
impl JointPoseAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
