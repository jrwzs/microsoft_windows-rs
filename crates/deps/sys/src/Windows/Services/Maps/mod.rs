#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Services_Maps_Guidance")]
pub mod Guidance;
#[cfg(feature = "Services_Maps_LocalSearch")]
pub mod LocalSearch;
#[cfg(feature = "Services_Maps_OfflineMaps")]
pub mod OfflineMaps;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EnhancedWaypoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnhancedWaypoint {}
impl ::core::clone::Clone for EnhancedWaypoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnhancedWaypoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnhancedWaypoint {}
impl ::core::clone::Clone for IEnhancedWaypoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnhancedWaypointFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnhancedWaypointFactory {}
impl ::core::clone::Clone for IEnhancedWaypointFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManeuverWarning(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManeuverWarning {}
impl ::core::clone::Clone for IManeuverWarning {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapAddress {}
impl ::core::clone::Clone for IMapAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapAddress2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapAddress2 {}
impl ::core::clone::Clone for IMapAddress2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapLocation {}
impl ::core::clone::Clone for IMapLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapLocationFinderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapLocationFinderResult {}
impl ::core::clone::Clone for IMapLocationFinderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapLocationFinderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapLocationFinderStatics {}
impl ::core::clone::Clone for IMapLocationFinderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapLocationFinderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapLocationFinderStatics2 {}
impl ::core::clone::Clone for IMapLocationFinderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapManagerStatics {}
impl ::core::clone::Clone for IMapManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRoute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRoute {}
impl ::core::clone::Clone for IMapRoute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRoute2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRoute2 {}
impl ::core::clone::Clone for IMapRoute2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRoute3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRoute3 {}
impl ::core::clone::Clone for IMapRoute3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRoute4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRoute4 {}
impl ::core::clone::Clone for IMapRoute4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteDrivingOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteDrivingOptions {}
impl ::core::clone::Clone for IMapRouteDrivingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteDrivingOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteDrivingOptions2 {}
impl ::core::clone::Clone for IMapRouteDrivingOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteFinderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteFinderResult {}
impl ::core::clone::Clone for IMapRouteFinderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteFinderResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteFinderResult2 {}
impl ::core::clone::Clone for IMapRouteFinderResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteFinderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteFinderStatics {}
impl ::core::clone::Clone for IMapRouteFinderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteFinderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteFinderStatics2 {}
impl ::core::clone::Clone for IMapRouteFinderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteFinderStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteFinderStatics3 {}
impl ::core::clone::Clone for IMapRouteFinderStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteLeg(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteLeg {}
impl ::core::clone::Clone for IMapRouteLeg {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteLeg2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteLeg2 {}
impl ::core::clone::Clone for IMapRouteLeg2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteManeuver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteManeuver {}
impl ::core::clone::Clone for IMapRouteManeuver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteManeuver2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteManeuver2 {}
impl ::core::clone::Clone for IMapRouteManeuver2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapRouteManeuver3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapRouteManeuver3 {}
impl ::core::clone::Clone for IMapRouteManeuver3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapServiceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapServiceStatics {}
impl ::core::clone::Clone for IMapServiceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapServiceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapServiceStatics2 {}
impl ::core::clone::Clone for IMapServiceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapServiceStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapServiceStatics3 {}
impl ::core::clone::Clone for IMapServiceStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapServiceStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapServiceStatics4 {}
impl ::core::clone::Clone for IMapServiceStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaceInfo {}
impl ::core::clone::Clone for IPlaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaceInfoCreateOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaceInfoCreateOptions {}
impl ::core::clone::Clone for IPlaceInfoCreateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaceInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaceInfoStatics {}
impl ::core::clone::Clone for IPlaceInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaceInfoStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaceInfoStatics2 {}
impl ::core::clone::Clone for IPlaceInfoStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManeuverWarning(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ManeuverWarning {}
impl ::core::clone::Clone for ManeuverWarning {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManeuverWarningKind(pub i32);
impl ManeuverWarningKind {
    pub const None: Self = Self(0i32);
    pub const Accident: Self = Self(1i32);
    pub const AdministrativeDivisionChange: Self = Self(2i32);
    pub const Alert: Self = Self(3i32);
    pub const BlockedRoad: Self = Self(4i32);
    pub const CheckTimetable: Self = Self(5i32);
    pub const Congestion: Self = Self(6i32);
    pub const Construction: Self = Self(7i32);
    pub const CountryChange: Self = Self(8i32);
    pub const DisabledVehicle: Self = Self(9i32);
    pub const GateAccess: Self = Self(10i32);
    pub const GetOffTransit: Self = Self(11i32);
    pub const GetOnTransit: Self = Self(12i32);
    pub const IllegalUTurn: Self = Self(13i32);
    pub const MassTransit: Self = Self(14i32);
    pub const Miscellaneous: Self = Self(15i32);
    pub const NoIncident: Self = Self(16i32);
    pub const Other: Self = Self(17i32);
    pub const OtherNews: Self = Self(18i32);
    pub const OtherTrafficIncidents: Self = Self(19i32);
    pub const PlannedEvent: Self = Self(20i32);
    pub const PrivateRoad: Self = Self(21i32);
    pub const RestrictedTurn: Self = Self(22i32);
    pub const RoadClosures: Self = Self(23i32);
    pub const RoadHazard: Self = Self(24i32);
    pub const ScheduledConstruction: Self = Self(25i32);
    pub const SeasonalClosures: Self = Self(26i32);
    pub const Tollbooth: Self = Self(27i32);
    pub const TollRoad: Self = Self(28i32);
    pub const TollZoneEnter: Self = Self(29i32);
    pub const TollZoneExit: Self = Self(30i32);
    pub const TrafficFlow: Self = Self(31i32);
    pub const TransitLineChange: Self = Self(32i32);
    pub const UnpavedRoad: Self = Self(33i32);
    pub const UnscheduledConstruction: Self = Self(34i32);
    pub const Weather: Self = Self(35i32);
}
impl ::core::marker::Copy for ManeuverWarningKind {}
impl ::core::clone::Clone for ManeuverWarningKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManeuverWarningSeverity(pub i32);
impl ManeuverWarningSeverity {
    pub const None: Self = Self(0i32);
    pub const LowImpact: Self = Self(1i32);
    pub const Minor: Self = Self(2i32);
    pub const Moderate: Self = Self(3i32);
    pub const Serious: Self = Self(4i32);
}
impl ::core::marker::Copy for ManeuverWarningSeverity {}
impl ::core::clone::Clone for ManeuverWarningSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapAddress {}
impl ::core::clone::Clone for MapAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapLocation {}
impl ::core::clone::Clone for MapLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapLocationDesiredAccuracy(pub i32);
impl MapLocationDesiredAccuracy {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for MapLocationDesiredAccuracy {}
impl ::core::clone::Clone for MapLocationDesiredAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapLocationFinderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapLocationFinderResult {}
impl ::core::clone::Clone for MapLocationFinderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapLocationFinderStatus(pub i32);
impl MapLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const BadLocation: Self = Self(3i32);
    pub const IndexFailure: Self = Self(4i32);
    pub const NetworkFailure: Self = Self(5i32);
    pub const NotSupported: Self = Self(6i32);
}
impl ::core::marker::Copy for MapLocationFinderStatus {}
impl ::core::clone::Clone for MapLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapManeuverNotices(pub u32);
impl MapManeuverNotices {
    pub const None: Self = Self(0u32);
    pub const Toll: Self = Self(1u32);
    pub const Unpaved: Self = Self(2u32);
}
impl ::core::marker::Copy for MapManeuverNotices {}
impl ::core::clone::Clone for MapManeuverNotices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRoute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapRoute {}
impl ::core::clone::Clone for MapRoute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteDrivingOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapRouteDrivingOptions {}
impl ::core::clone::Clone for MapRouteDrivingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteFinderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapRouteFinderResult {}
impl ::core::clone::Clone for MapRouteFinderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteFinderStatus(pub i32);
impl MapRouteFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NoRouteFound: Self = Self(3i32);
    pub const NoRouteFoundWithGivenOptions: Self = Self(4i32);
    pub const StartPointNotFound: Self = Self(5i32);
    pub const EndPointNotFound: Self = Self(6i32);
    pub const NoPedestrianRouteFound: Self = Self(7i32);
    pub const NetworkFailure: Self = Self(8i32);
    pub const NotSupported: Self = Self(9i32);
}
impl ::core::marker::Copy for MapRouteFinderStatus {}
impl ::core::clone::Clone for MapRouteFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteLeg(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapRouteLeg {}
impl ::core::clone::Clone for MapRouteLeg {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteManeuver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapRouteManeuver {}
impl ::core::clone::Clone for MapRouteManeuver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteManeuverKind(pub i32);
impl MapRouteManeuverKind {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Stopover: Self = Self(2i32);
    pub const StopoverResume: Self = Self(3i32);
    pub const End: Self = Self(4i32);
    pub const GoStraight: Self = Self(5i32);
    pub const UTurnLeft: Self = Self(6i32);
    pub const UTurnRight: Self = Self(7i32);
    pub const TurnKeepLeft: Self = Self(8i32);
    pub const TurnKeepRight: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLightRight: Self = Self(11i32);
    pub const TurnLeft: Self = Self(12i32);
    pub const TurnRight: Self = Self(13i32);
    pub const TurnHardLeft: Self = Self(14i32);
    pub const TurnHardRight: Self = Self(15i32);
    pub const FreewayEnterLeft: Self = Self(16i32);
    pub const FreewayEnterRight: Self = Self(17i32);
    pub const FreewayLeaveLeft: Self = Self(18i32);
    pub const FreewayLeaveRight: Self = Self(19i32);
    pub const FreewayContinueLeft: Self = Self(20i32);
    pub const FreewayContinueRight: Self = Self(21i32);
    pub const TrafficCircleLeft: Self = Self(22i32);
    pub const TrafficCircleRight: Self = Self(23i32);
    pub const TakeFerry: Self = Self(24i32);
}
impl ::core::marker::Copy for MapRouteManeuverKind {}
impl ::core::clone::Clone for MapRouteManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteOptimization(pub i32);
impl MapRouteOptimization {
    pub const Time: Self = Self(0i32);
    pub const Distance: Self = Self(1i32);
    pub const TimeWithTraffic: Self = Self(2i32);
    pub const Scenic: Self = Self(3i32);
}
impl ::core::marker::Copy for MapRouteOptimization {}
impl ::core::clone::Clone for MapRouteOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapRouteRestrictions(pub u32);
impl MapRouteRestrictions {
    pub const None: Self = Self(0u32);
    pub const Highways: Self = Self(1u32);
    pub const TollRoads: Self = Self(2u32);
    pub const Ferries: Self = Self(4u32);
    pub const Tunnels: Self = Self(8u32);
    pub const DirtRoads: Self = Self(16u32);
    pub const Motorail: Self = Self(32u32);
}
impl ::core::marker::Copy for MapRouteRestrictions {}
impl ::core::clone::Clone for MapRouteRestrictions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapServiceDataUsagePreference(pub i32);
impl MapServiceDataUsagePreference {
    pub const Default: Self = Self(0i32);
    pub const OfflineMapDataOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MapServiceDataUsagePreference {}
impl ::core::clone::Clone for MapServiceDataUsagePreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaceInfo {}
impl ::core::clone::Clone for PlaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaceInfoCreateOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaceInfoCreateOptions {}
impl ::core::clone::Clone for PlaceInfoCreateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TrafficCongestion(pub i32);
impl TrafficCongestion {
    pub const Unknown: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Mild: Self = Self(2i32);
    pub const Medium: Self = Self(3i32);
    pub const Heavy: Self = Self(4i32);
}
impl ::core::marker::Copy for TrafficCongestion {}
impl ::core::clone::Clone for TrafficCongestion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WaypointKind(pub i32);
impl WaypointKind {
    pub const Stop: Self = Self(0i32);
    pub const Via: Self = Self(1i32);
}
impl ::core::marker::Copy for WaypointKind {}
impl ::core::clone::Clone for WaypointKind {
    fn clone(&self) -> Self {
        *self
    }
}
