#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: Self = Self(0i32);
}
#[repr(transparent)]
pub struct AdaptiveNotificationText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BadgeNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: Self = Self(0i32);
    pub const BadgeNumber: Self = Self(1i32);
}
#[repr(transparent)]
pub struct BadgeUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BadgeUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveNotificationContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveNotificationText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationHintsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationTextStylesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownNotificationBindingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationDataFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledTileNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotificationShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShownTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutUpdateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdateManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdater2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastActivatedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastCollectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastCollectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastDismissedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationActionTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerForUser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotifier2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotifier3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotificationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Notification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: Self = Self(0u32);
    pub const Toast: Self = Self(1u32);
}
#[repr(transparent)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
#[repr(transparent)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
}
#[repr(transparent)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const NotificationNotFound: Self = Self(2i32);
}
#[repr(transparent)]
pub struct NotificationVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const SixHours: Self = Self(2i32);
    pub const TwelveHours: Self = Self(3i32);
    pub const Daily: Self = Self(4i32);
}
#[repr(transparent)]
pub struct ScheduledTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScheduledToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScheduledToastNotificationShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShownTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileFlyoutNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: Self = Self(0i32);
}
#[repr(transparent)]
pub struct TileFlyoutUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: Self = Self(0i32);
    pub const TileSquareBlock: Self = Self(1i32);
    pub const TileSquareText01: Self = Self(2i32);
    pub const TileSquareText02: Self = Self(3i32);
    pub const TileSquareText03: Self = Self(4i32);
    pub const TileSquareText04: Self = Self(5i32);
    pub const TileSquarePeekImageAndText01: Self = Self(6i32);
    pub const TileSquarePeekImageAndText02: Self = Self(7i32);
    pub const TileSquarePeekImageAndText03: Self = Self(8i32);
    pub const TileSquarePeekImageAndText04: Self = Self(9i32);
    pub const TileWideImage: Self = Self(10i32);
    pub const TileWideImageCollection: Self = Self(11i32);
    pub const TileWideImageAndText01: Self = Self(12i32);
    pub const TileWideImageAndText02: Self = Self(13i32);
    pub const TileWideBlockAndText01: Self = Self(14i32);
    pub const TileWideBlockAndText02: Self = Self(15i32);
    pub const TileWidePeekImageCollection01: Self = Self(16i32);
    pub const TileWidePeekImageCollection02: Self = Self(17i32);
    pub const TileWidePeekImageCollection03: Self = Self(18i32);
    pub const TileWidePeekImageCollection04: Self = Self(19i32);
    pub const TileWidePeekImageCollection05: Self = Self(20i32);
    pub const TileWidePeekImageCollection06: Self = Self(21i32);
    pub const TileWidePeekImageAndText01: Self = Self(22i32);
    pub const TileWidePeekImageAndText02: Self = Self(23i32);
    pub const TileWidePeekImage01: Self = Self(24i32);
    pub const TileWidePeekImage02: Self = Self(25i32);
    pub const TileWidePeekImage03: Self = Self(26i32);
    pub const TileWidePeekImage04: Self = Self(27i32);
    pub const TileWidePeekImage05: Self = Self(28i32);
    pub const TileWidePeekImage06: Self = Self(29i32);
    pub const TileWideSmallImageAndText01: Self = Self(30i32);
    pub const TileWideSmallImageAndText02: Self = Self(31i32);
    pub const TileWideSmallImageAndText03: Self = Self(32i32);
    pub const TileWideSmallImageAndText04: Self = Self(33i32);
    pub const TileWideSmallImageAndText05: Self = Self(34i32);
    pub const TileWideText01: Self = Self(35i32);
    pub const TileWideText02: Self = Self(36i32);
    pub const TileWideText03: Self = Self(37i32);
    pub const TileWideText04: Self = Self(38i32);
    pub const TileWideText05: Self = Self(39i32);
    pub const TileWideText06: Self = Self(40i32);
    pub const TileWideText07: Self = Self(41i32);
    pub const TileWideText08: Self = Self(42i32);
    pub const TileWideText09: Self = Self(43i32);
    pub const TileWideText10: Self = Self(44i32);
    pub const TileWideText11: Self = Self(45i32);
    pub const TileSquare150x150Image: Self = Self(0i32);
    pub const TileSquare150x150Block: Self = Self(1i32);
    pub const TileSquare150x150Text01: Self = Self(2i32);
    pub const TileSquare150x150Text02: Self = Self(3i32);
    pub const TileSquare150x150Text03: Self = Self(4i32);
    pub const TileSquare150x150Text04: Self = Self(5i32);
    pub const TileSquare150x150PeekImageAndText01: Self = Self(6i32);
    pub const TileSquare150x150PeekImageAndText02: Self = Self(7i32);
    pub const TileSquare150x150PeekImageAndText03: Self = Self(8i32);
    pub const TileSquare150x150PeekImageAndText04: Self = Self(9i32);
    pub const TileWide310x150Image: Self = Self(10i32);
    pub const TileWide310x150ImageCollection: Self = Self(11i32);
    pub const TileWide310x150ImageAndText01: Self = Self(12i32);
    pub const TileWide310x150ImageAndText02: Self = Self(13i32);
    pub const TileWide310x150BlockAndText01: Self = Self(14i32);
    pub const TileWide310x150BlockAndText02: Self = Self(15i32);
    pub const TileWide310x150PeekImageCollection01: Self = Self(16i32);
    pub const TileWide310x150PeekImageCollection02: Self = Self(17i32);
    pub const TileWide310x150PeekImageCollection03: Self = Self(18i32);
    pub const TileWide310x150PeekImageCollection04: Self = Self(19i32);
    pub const TileWide310x150PeekImageCollection05: Self = Self(20i32);
    pub const TileWide310x150PeekImageCollection06: Self = Self(21i32);
    pub const TileWide310x150PeekImageAndText01: Self = Self(22i32);
    pub const TileWide310x150PeekImageAndText02: Self = Self(23i32);
    pub const TileWide310x150PeekImage01: Self = Self(24i32);
    pub const TileWide310x150PeekImage02: Self = Self(25i32);
    pub const TileWide310x150PeekImage03: Self = Self(26i32);
    pub const TileWide310x150PeekImage04: Self = Self(27i32);
    pub const TileWide310x150PeekImage05: Self = Self(28i32);
    pub const TileWide310x150PeekImage06: Self = Self(29i32);
    pub const TileWide310x150SmallImageAndText01: Self = Self(30i32);
    pub const TileWide310x150SmallImageAndText02: Self = Self(31i32);
    pub const TileWide310x150SmallImageAndText03: Self = Self(32i32);
    pub const TileWide310x150SmallImageAndText04: Self = Self(33i32);
    pub const TileWide310x150SmallImageAndText05: Self = Self(34i32);
    pub const TileWide310x150Text01: Self = Self(35i32);
    pub const TileWide310x150Text02: Self = Self(36i32);
    pub const TileWide310x150Text03: Self = Self(37i32);
    pub const TileWide310x150Text04: Self = Self(38i32);
    pub const TileWide310x150Text05: Self = Self(39i32);
    pub const TileWide310x150Text06: Self = Self(40i32);
    pub const TileWide310x150Text07: Self = Self(41i32);
    pub const TileWide310x150Text08: Self = Self(42i32);
    pub const TileWide310x150Text09: Self = Self(43i32);
    pub const TileWide310x150Text10: Self = Self(44i32);
    pub const TileWide310x150Text11: Self = Self(45i32);
    pub const TileSquare310x310BlockAndText01: Self = Self(46i32);
    pub const TileSquare310x310BlockAndText02: Self = Self(47i32);
    pub const TileSquare310x310Image: Self = Self(48i32);
    pub const TileSquare310x310ImageAndText01: Self = Self(49i32);
    pub const TileSquare310x310ImageAndText02: Self = Self(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: Self = Self(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: Self = Self(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: Self = Self(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: Self = Self(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: Self = Self(55i32);
    pub const TileSquare310x310ImageCollection: Self = Self(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: Self = Self(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: Self = Self(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: Self = Self(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: Self = Self(60i32);
    pub const TileSquare310x310Text01: Self = Self(61i32);
    pub const TileSquare310x310Text02: Self = Self(62i32);
    pub const TileSquare310x310Text03: Self = Self(63i32);
    pub const TileSquare310x310Text04: Self = Self(64i32);
    pub const TileSquare310x310Text05: Self = Self(65i32);
    pub const TileSquare310x310Text06: Self = Self(66i32);
    pub const TileSquare310x310Text07: Self = Self(67i32);
    pub const TileSquare310x310Text08: Self = Self(68i32);
    pub const TileSquare310x310TextList01: Self = Self(69i32);
    pub const TileSquare310x310TextList02: Self = Self(70i32);
    pub const TileSquare310x310TextList03: Self = Self(71i32);
    pub const TileSquare310x310SmallImageAndText01: Self = Self(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: Self = Self(73i32);
    pub const TileSquare310x310Text09: Self = Self(74i32);
    pub const TileSquare71x71IconWithBadge: Self = Self(75i32);
    pub const TileSquare150x150IconWithBadge: Self = Self(76i32);
    pub const TileWide310x150IconWithBadgeAndText: Self = Self(77i32);
    pub const TileSquare71x71Image: Self = Self(78i32);
    pub const TileTall150x310Image: Self = Self(79i32);
}
#[repr(transparent)]
pub struct TileUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastCollectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: Self = Self(0i32);
    pub const ApplicationHidden: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ToastDismissedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Expired: Self = Self(2i32);
    pub const Added: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationActionTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ToastNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: Self = Self(0i32);
    pub const ToastImageAndText02: Self = Self(1i32);
    pub const ToastImageAndText03: Self = Self(2i32);
    pub const ToastImageAndText04: Self = Self(3i32);
    pub const ToastText01: Self = Self(4i32);
    pub const ToastText02: Self = Self(5i32);
    pub const ToastText03: Self = Self(6i32);
    pub const ToastText04: Self = Self(7i32);
}
#[repr(transparent)]
pub struct UserNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
}
