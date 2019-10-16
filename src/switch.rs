//! Route based on enums.
use crate::route::Route;
use crate::RouteState;
use std::fmt::Write;

/// Routing trait for enums.
///
/// # Note
/// Don't try to implement this yourself, rely on the derive macro.
///
/// If you want to dictate how an item is serialized/deserialized to a route string, implement `RouteItem` instead.
///
/// # Example
/// ```
/// use yew_router::Switch;
/// use yew_router::route::Route;
/// #[derive(Debug, Switch, PartialEq)]
/// enum TestEnum {
///     #[to = "/test/route"]
///     TestRoute,
///     #[to = "/capture/string/{path}"]
///     CaptureString{path: String},
///     #[to = "/capture/number/{num}"]
///     CaptureNumber{num: usize},
///     #[to = "/capture/unnamed/{doot}"]
///     CaptureUnnamed(String),
///     #[to = "{*}/skip/"]
///     Skip
/// }
///
/// assert_eq!(TestEnum::switch(Route::<()>::from("/test/route")), Some(TestEnum::TestRoute));
/// assert_eq!(TestEnum::switch(Route::<()>::from("/capture/string/lorem")), Some(TestEnum::CaptureString{path: "lorem".to_string()}));
/// assert_eq!(TestEnum::switch(Route::<()>::from("/capture/number/22")), Some(TestEnum::CaptureNumber{num: 22}));
/// assert_eq!(TestEnum::switch(Route::<()>::from("/capture/unnamed/lorem")), Some(TestEnum::CaptureUnnamed("lorem".to_string())));
/// ```
///
pub trait Switch: Sized {
    /// Based on a route, possibly produce an itself.
    fn switch<T: RouteState>(route: Route<T>) -> Option<Self> {
        Self::from_route_part(route).0
    }

    /// Get self from a part of the state
    fn from_route_part<T: RouteState>(part: Route<T>) -> (Option<Self>, Option<T>);

    /// Build part of a route from itself.
    fn build_route_section<T>(self, route: &mut String) -> Option<T>;

    /// Called when the key (the named capture group) can't be located. Instead of failing outright, a default item can be provided instead.
    ///
    /// Its primary motivation for existing is to allow implementing Switch for Option.
    /// This doesn't make sense at the moment because this only works for the individual key section - any surrounding literals are pretty much guaranteed to make the parse step fail.
    /// because of this, this functionality might be removed in favor of using a nested Switch enum, or multiple variants.
    fn key_not_available() -> Option<Self> {
        None
    }
}

// Ok, the pattern is:
// Provide route
// For each match, pull the expected item out of the hashmap,
// create a Route comprised of that item, and the state.
// Rescue the state afterwords to move it into the next item.

/// Builds a route from a switch.
pub fn build_route_from_switch<T: Switch, U>(switch: T) -> Route<U> {
    let mut buf = String::with_capacity(50); // TODO, play with this to maximize perf/size.

    let state: Option<U> = None;
    let state = state.or(switch.build_route_section(&mut buf));
    Route { route: buf, state }
}

//impl <U: RouteItem> RouteItem for Option<U> {
//    fn from_route_part<T: RouteState>(part: &mut Route<T>) -> Option<Self> {
//        Some(Some(RouteItem::from_route_part(part)?)) // TODO not 100% sure about the semantics of this.
//    }
//
//    fn build_route_section<T>(self, f: &mut String) -> Option<T> {
//        if let Some(item) = self {
//            item.build_route_section(f)
//        } else {
//            None
//        }
//    }
//
//    fn key_not_available() -> Option<Self> {
//        Some(None)
//    }
//}

/// Wrapper that allows any implementor of RouteItem to be treated as a switch.
/// It stipulates that the first character must be a slash.
//pub struct LeadingSlash<T>(pub T);
//impl <U: RouteItem> Switch for LeadingSlash<U> {
//    fn switch<T: RouteState>(route: Route<T>) -> (Option<Self>, Route<T>) {
//        unimplemented!()
//    }
//
//    fn build_route_section<T>(self, route: &mut String) -> Option<T> {
//        write!(f, "/{}", self.0).ok()?;
//        None
//    }
//}

macro_rules! impl_switch_for_from_to_str {
    ($($SelfT: ty),*) => {
        $(
        impl Switch for $SelfT {
            fn from_route_part<T: RouteState>(part: Route<T>) -> (Option<Self>, Option<T>) {
                (
                    ::std::str::FromStr::from_str(&part.route).ok(),
                    part.state
                )
            }

            fn build_route_section<T>(self, f: &mut String) -> Option<T> {
                write!(f, "{}", self).ok()?; // TODO no idea if this works or not.
                None
            }
        }
        )*
    };
}

impl_switch_for_from_to_str! {
    String,
    bool,
    f64,
    f32,
    usize,
    u128,
    u64,
    u32,
    u16,
    u8,
    isize,
    i128,
    i64,
    i32,
    i16,
    i8,
    std::num::NonZeroU128,
    std::num::NonZeroU64,
    std::num::NonZeroU32,
    std::num::NonZeroU16,
    std::num::NonZeroU8,
    std::num::NonZeroI128,
    std::num::NonZeroI64,
    std::num::NonZeroI32,
    std::num::NonZeroI16,
    std::num::NonZeroI8
}

#[test]
fn isize_build_route() {
    let mut route = "/".to_string();
    let mut state: Option<String> = None;
    state.or((-432isize).build_route_section(&mut route));
    assert_eq!(route, "/-432".to_string());
}

//impl<U: Switch> Switch for Option<U> {
//    fn switch<T: RouteState>(route: Route<T>) -> Option<Self> {
//        Some(Some(Switch::switch(route)?))
//    }
//
//    /// This will cause the derivation of `from_matches` to not fail if the key can't be located
//    fn key_not_available() -> Option<Self> {
//        Some(None)
//    }
//}

//impl<U, E> Switch for Result<U, E>
//where
//    U: FromStr<Err = E>,
//{
//    fn switch<T: RouteState>(route: Route<T>) -> Option<Self> {
//        Some(U::from_str(&route.route))
//    }
//}
//
//macro_rules! impl_switch_for_from_str {
//    ($($SelfT: ty),*) => {
//        $(
//        impl Switch for $SelfT {
//            fn switch<T>(route: Route<T>) -> Option<Self> {
//                std::str::FromStr::from_str(&route.route).ok()
//            }
//        }
//        )*
//    };
//}

// TODO add implementations for Dates - with various formats, UUIDs
//impl_switch_for_from_str! {
//    String,
//    PathBuf,
//    bool,
//    f64,
//    f32,
//    usize,
//    u128,
//    u64,
//    u32,
//    u16,
//    u8,
//    isize,
//    i128,
//    i64,
//    i32,
//    i16,
//    i8,
//    std::num::NonZeroU128,
//    std::num::NonZeroU64,
//    std::num::NonZeroU32,
//    std::num::NonZeroU16,
//    std::num::NonZeroU8,
//    std::num::NonZeroI128,
//    std::num::NonZeroI64,
//    std::num::NonZeroI32,
//    std::num::NonZeroI16,
//    std::num::NonZeroI8
//}
