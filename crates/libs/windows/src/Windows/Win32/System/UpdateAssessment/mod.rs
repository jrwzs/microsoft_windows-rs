::windows_core::imp::com_interface!(IWaaSAssessor, IWaaSAssessor_Vtbl, 0x2347bbef_1a3b_45a4_902d_3e09c269b45e);
::windows_core::imp::interface_hierarchy!(IWaaSAssessor, ::windows_core::IUnknown);
impl IWaaSAssessor {
    pub unsafe fn GetOSUpdateAssessment(&self) -> ::windows_core::Result<OSUpdateAssessment> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOSUpdateAssessment)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaaSAssessor_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOSUpdateAssessment: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut OSUpdateAssessment) -> ::windows_core::HRESULT,
}
pub const UpdateAssessmentStatus_Latest: UpdateAssessmentStatus = UpdateAssessmentStatus(0i32);
pub const UpdateAssessmentStatus_NotLatestDeferredFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(5i32);
pub const UpdateAssessmentStatus_NotLatestDeferredQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(6i32);
pub const UpdateAssessmentStatus_NotLatestEndOfSupport: UpdateAssessmentStatus = UpdateAssessmentStatus(3i32);
pub const UpdateAssessmentStatus_NotLatestHardRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(2i32);
pub const UpdateAssessmentStatus_NotLatestManaged: UpdateAssessmentStatus = UpdateAssessmentStatus(9i32);
pub const UpdateAssessmentStatus_NotLatestPausedFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(7i32);
pub const UpdateAssessmentStatus_NotLatestPausedQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(8i32);
pub const UpdateAssessmentStatus_NotLatestServicingTrain: UpdateAssessmentStatus = UpdateAssessmentStatus(4i32);
pub const UpdateAssessmentStatus_NotLatestSoftRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(1i32);
pub const UpdateAssessmentStatus_NotLatestTargetedVersion: UpdateAssessmentStatus = UpdateAssessmentStatus(11i32);
pub const UpdateAssessmentStatus_NotLatestUnknown: UpdateAssessmentStatus = UpdateAssessmentStatus(10i32);
pub const UpdateImpactLevel_High: UpdateImpactLevel = UpdateImpactLevel(3i32);
pub const UpdateImpactLevel_Low: UpdateImpactLevel = UpdateImpactLevel(1i32);
pub const UpdateImpactLevel_Medium: UpdateImpactLevel = UpdateImpactLevel(2i32);
pub const UpdateImpactLevel_None: UpdateImpactLevel = UpdateImpactLevel(0i32);
pub const WaaSAssessor: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x098ef871_fa9f_46af_8958_c083515d7c9c);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct UpdateAssessmentStatus(pub i32);
impl ::windows_core::TypeKind for UpdateAssessmentStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UpdateAssessmentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateAssessmentStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct UpdateImpactLevel(pub i32);
impl ::windows_core::TypeKind for UpdateImpactLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UpdateImpactLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateImpactLevel").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct OSUpdateAssessment {
    pub isEndOfSupport: super::super::Foundation::BOOL,
    pub assessmentForCurrent: UpdateAssessment,
    pub assessmentForUpToDate: UpdateAssessment,
    pub securityStatus: UpdateAssessmentStatus,
    pub assessmentTime: super::super::Foundation::FILETIME,
    pub releaseInfoTime: super::super::Foundation::FILETIME,
    pub currentOSBuild: ::windows_core::PWSTR,
    pub currentOSReleaseTime: super::super::Foundation::FILETIME,
    pub upToDateOSBuild: ::windows_core::PWSTR,
    pub upToDateOSReleaseTime: super::super::Foundation::FILETIME,
}
impl ::core::marker::Copy for OSUpdateAssessment {}
impl ::core::clone::Clone for OSUpdateAssessment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSUpdateAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSUpdateAssessment")
            .field("isEndOfSupport", &self.isEndOfSupport)
            .field("assessmentForCurrent", &self.assessmentForCurrent)
            .field("assessmentForUpToDate", &self.assessmentForUpToDate)
            .field("securityStatus", &self.securityStatus)
            .field("assessmentTime", &self.assessmentTime)
            .field("releaseInfoTime", &self.releaseInfoTime)
            .field("currentOSBuild", &self.currentOSBuild)
            .field("currentOSReleaseTime", &self.currentOSReleaseTime)
            .field("upToDateOSBuild", &self.upToDateOSBuild)
            .field("upToDateOSReleaseTime", &self.upToDateOSReleaseTime)
            .finish()
    }
}
impl ::windows_core::TypeKind for OSUpdateAssessment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OSUpdateAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.isEndOfSupport == other.isEndOfSupport && self.assessmentForCurrent == other.assessmentForCurrent && self.assessmentForUpToDate == other.assessmentForUpToDate && self.securityStatus == other.securityStatus && self.assessmentTime == other.assessmentTime && self.releaseInfoTime == other.releaseInfoTime && self.currentOSBuild == other.currentOSBuild && self.currentOSReleaseTime == other.currentOSReleaseTime && self.upToDateOSBuild == other.upToDateOSBuild && self.upToDateOSReleaseTime == other.upToDateOSReleaseTime
    }
}
impl ::core::cmp::Eq for OSUpdateAssessment {}
impl ::core::default::Default for OSUpdateAssessment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct UpdateAssessment {
    pub status: UpdateAssessmentStatus,
    pub impact: UpdateImpactLevel,
    pub daysOutOfDate: u32,
}
impl ::core::marker::Copy for UpdateAssessment {}
impl ::core::clone::Clone for UpdateAssessment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UpdateAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UpdateAssessment").field("status", &self.status).field("impact", &self.impact).field("daysOutOfDate", &self.daysOutOfDate).finish()
    }
}
impl ::windows_core::TypeKind for UpdateAssessment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for UpdateAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.impact == other.impact && self.daysOutOfDate == other.daysOutOfDate
    }
}
impl ::core::cmp::Eq for UpdateAssessment {}
impl ::core::default::Default for UpdateAssessment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
