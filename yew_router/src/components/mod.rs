//! Components that integrate with the [route agent](struct.RouteAgent.html).
//!
pub mod router_button;
pub mod router_link;
pub mod route_injector;



pub use self::router_button::RouterButton;
pub use self::router_link::RouterLink;

/// Alias to 'RouteInjector<(), C>'
pub type RouteInjector<C> = self::route_injector::RouteInjector<(), C>;
/// Trait alias to 'RouteInjectable<()>'
pub trait RouteInjectable: self::route_injector::RouteInjectable<()> {}



use yew::Properties;
/// Properties for Routing Components
#[derive(Properties, Default, Clone, Debug, PartialEq)]
pub struct Props {
    /// The route that will be set when the component is clicked.
    pub link: String,
    /// The state to set when changing the route.
    pub state: Option<()>,
    /// The text to display.
    pub text: String,
    /// Disable the component.
    pub disabled: bool,
    /// Classes to be added to component.
    pub classes: String,
}

/// Message for Routing Components.
#[derive(Clone, Copy, Debug)]
pub enum Msg {
    /// Perform no action
    NoOp,
    /// Tell the router to navigate the application to the Component's pre-defined route.
    Clicked,
}

