//! Route based on enums.
use crate::route_info::RouteInfo;

/// Routing trait for enums
pub trait Switch: Sized {
    /// Based on a route, possibly produce an itself.
    fn switch<T>(route: RouteInfo<T>) -> Option<Self>;
}