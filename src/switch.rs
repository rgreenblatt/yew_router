use crate::route_info::RouteInfo;

pub trait Switch {
    fn switch<T>(route: RouteInfo<T>) -> Option<Self>;
}