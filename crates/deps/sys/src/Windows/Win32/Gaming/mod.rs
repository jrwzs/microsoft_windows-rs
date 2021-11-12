#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilentlyForUser(user: ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CheckGamingPrivilegeWithUIForUser(user: ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn ReleaseExclusiveCpuSets() -> ::windows_sys::core::HRESULT;
    pub fn ShowChangeFriendRelationshipUI(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowChangeFriendRelationshipUIForUser(user: ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowCustomizeUserProfileUIForUser(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowFindFriendsUIForUser(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInfoUIForUser(user: ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUI(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUIForUser(user: ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUIWithContext(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUIWithContextForUser(user: ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowPlayerPickerUI(promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowPlayerPickerUIForUser(user: ::windows_sys::core::IInspectable, promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowProfileCardUI(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowProfileCardUIForUser(user: ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowTitleAchievementsUIForUser(user: ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowUserSettingsUIForUser(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
}
#[repr(transparent)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
#[repr(transparent)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
#[repr(transparent)]
pub struct GAME_INSTALL_SCOPE(pub i32);
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
#[repr(transparent)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
#[repr(C)]
pub struct GAMING_DEVICE_MODEL_INFORMATION(i32);
#[repr(transparent)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
#[repr(C)]
pub struct GameExplorer(i32);
#[repr(C)]
pub struct GameStatistics(i32);
pub type GameUICompletionRoutine = unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameExplorer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameExplorer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameStatisticsMgr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXblIdpAuthManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownGamingPrivileges(pub i32);
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = KnownGamingPrivileges(190i32);
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = KnownGamingPrivileges(197i32);
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = KnownGamingPrivileges(198i32);
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(199i32);
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = KnownGamingPrivileges(203i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = KnownGamingPrivileges(205i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = KnownGamingPrivileges(206i32);
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(207i32);
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(208i32);
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = KnownGamingPrivileges(209i32);
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(211i32);
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(214i32);
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(219i32);
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = KnownGamingPrivileges(220i32);
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = KnownGamingPrivileges(224i32);
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(235i32);
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(245i32);
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(247i32);
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = KnownGamingPrivileges(249i32);
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(252i32);
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = KnownGamingPrivileges(254i32);
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = KnownGamingPrivileges(255i32);
pub type PlayerPickerUICompletionRoutine = unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows_sys::core::HSTRING, selectedxuidscount: usize);
#[repr(transparent)]
pub struct XBL_IDP_AUTH_TOKEN_STATUS(pub i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(0i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(1i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(2i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(3i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(4i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(5i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(6i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(7i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(-1i32);
#[repr(C)]
pub struct XblIdpAuthManager(i32);
#[repr(C)]
pub struct XblIdpAuthTokenResult(i32);
