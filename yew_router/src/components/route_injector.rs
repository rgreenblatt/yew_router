//! A component that keeps track of the current route string and can modify its wrapped children via props
//! to indicate the route.
use crate::agent::{bridge::RouteAgentBridge, RouteRequest};
use crate::route_info::RouteInfo;
use crate::router_component::YewRouterState;
use std::fmt::{Debug, Error as FmtError, Formatter};
use yew::{
    ChildrenWithProps, Component, ComponentLink, Html, Properties, Renderable, ShouldRender,
};
use yew::virtual_dom::{VNode, VChild};

/// A trait allowing user-defined components to have their props rewritten by a parent `RouteInjector` when the route changes.
pub trait RouteInjectable<T: for<'de> YewRouterState<'de>>: Component + Renderable<Self> {
    /// Changes the props based on a route.
    ///
    ///
    /// # Example
    /// ```
    /// use yew_router::components::RouteInjectable;
    /// use yew_router::RouteInfo;
    ///
    /// impl RouteInjectable for ListElement {
    ///     fn inject_route(mut props: &mut Self::Properties, route_info: &RouteInfo) {
    ///          props.is_active = props.matcher.match_route_string(&route_info.route).is_some();
    ///     }
    /// }
    /// ```
    fn inject_route(&mut self, route_info: &RouteInfo<T>);
}

/// A component that wraps child components and can tell them what the route is via props.
///
/// # Example
/// ```
/// use yew_router::matcher::{Matcher, MatcherProvider};
/// # use yew::{Component, ComponentLink, Renderable, Html, Properties, html, Classes, Children};
/// use yew_router::components::{RouteInjectable, RouteInjector};
/// # use yew_router::{RouteInfo, route};
/// pub struct ListElement {
///     props: ListElementProps
/// }
/// #[derive(Properties)]
/// pub struct ListElementProps {
///     is_active: bool,
///     children: Children<ListElement>,
///     #[props(required)]
///     matcher: Matcher
/// }
/// impl Component for ListElement {
///#     type Message = ();
///     type Properties = ListElementProps;
///     // ...
///#
///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {
///#         unimplemented!()
///#     }
///#     fn update(&mut self,msg: Self::Message) -> bool {
///#         unimplemented!()
///#     }
/// }
/// impl Renderable<ListElement> for ListElement {
///     fn view(&self) -> Html<ListElement> {
///         let mut classes = Classes::new();
///         if self.props.is_active {
///             classes.push("active");
///         }
///         html!{
///             <li class=classes>
///                {self.props.children.iter().collect::<Html<ListElement>>()}
///             </li>
///         }
///     }
/// }
/// impl RouteInjectable for ListElement {
///     fn inject_route(mut props: &mut Self::Properties, route_info: &RouteInfo) {
///          props.is_active = props.matcher.match_route_string(&route_info.route).is_some();
///     }
/// }
///# pub struct Comp;
///# impl Component for Comp {type Message = ();type Properties = ();
///# fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
///# fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
///# }
///
/// fn view() -> Html<Comp> {
///     html! {
///         <ul>
///             <RouteInjector<ListElement>>
///                 <ListElement matcher = route!("/hi")> {"Hi"} </ListElement>
///                 <ListElement matcher = route!("/goodbye")>  {"Goodbye"} </ListElement>
///             </RouteInjector>
///         </ul>
///     }
/// }
///
///
/// ```
///
#[derive(Debug)]
pub struct RouteInjector<T, C>
where
    T: for<'de> YewRouterState<'de>,
    C: Component + Renderable<C>,
    <C as Component>::Properties: RouteInjectable<T>,
{
    router_bridge: RouteAgentBridge<T>,
    route_info: Option<RouteInfo<T>>,
    props: Props<T, C>,
}

/// Properties for `RouteInjector`.
#[derive(Properties)]
pub struct Props<T: for<'de> YewRouterState<'de>, C: Component + Renderable<C>>
where
    <C as Component>::Properties: RouteInjectable<T>
{
    children: ChildrenWithProps<C, RouteInjector<T, C>>,
}

impl<T, C> Debug for Props<T, C>
where
    T: for<'de> YewRouterState<'de>,
    C: Component + Renderable<C>,
    <C as Component>::Properties: RouteInjectable<T>,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_struct("Props")
            .field(
                "children",
                &"ChildrenWithProps<_, ActiveWrapper<_, _>".to_owned(),
            )
            .finish()
    }
}

/// Message type for `RouteInjector`.
#[derive(Debug)]
pub enum Msg<T: for<'de> YewRouterState<'de>> {
    /// Message indicating that the route has changed
    RouteUpdated(RouteInfo<T>),
}

impl<T, C> Component for RouteInjector<T, C>
where
    T: for<'de> YewRouterState<'de>,
    C: Component + Renderable<C>,
    <C as Component>::Properties: RouteInjectable<T>,
{
    type Message = Msg<T>;
    type Properties = Props<T, C>;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route_info| Msg::RouteUpdated(route_info));
        RouteInjector {
            router_bridge: RouteAgentBridge::new(callback),
            route_info: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.router_bridge.send(RouteRequest::GetCurrentRoute);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteUpdated(route_info) => self.route_info = Some(route_info),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl<T, C> Renderable<RouteInjector<T, C>> for RouteInjector<T, C>
where
    T: for<'de> YewRouterState<'de>,
    C: Component + Renderable<C>,
    <C as Component>::Properties: RouteInjectable<T>,
{
    fn view(&self) -> Html<Self> {
        self.props
            .children
            .iter()
            .map(|mut child| {
                if let Some(route_info) = &self.route_info {
                    // Allow the children to change their props based on the route.
                    child.props.inject_route(&route_info)
                }
                crate::router_component::render::create_component::<C, Self>(child.props)
            })
            .collect::<VNode<Self>>()
    }
}
