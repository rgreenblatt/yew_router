//! A component that keeps track of the current route string and can modify its wrapped children via props
//! to indicate the route.
use yew::{ChildrenWithProps, Component, ComponentLink, Renderable, Html, Properties, ShouldRender};
use crate::router_component::YewRouterState;
use crate::route_agent::{RouteRequest, RouteAgentBridge};
use crate::route_info::RouteInfo;
use std::fmt::{Debug, Formatter, Error as FmtError};

/// A trait allowing user-defined components to have their props rewritten by a parent ActiveWrapper when the route changes.
pub trait Activatable: Component + Renderable<Self> {
    /// Changes the props.
    fn set_active(props: Self::Properties, route_string: &str) -> Self::Properties;
}

/// A component that wraps child components and can tell them what the route is.
#[derive(Debug)]
pub struct ActiveWrapper<T, C>
where
    T: for<'de> YewRouterState<'de>,
    C: Activatable
{
    router_bridge: RouteAgentBridge<T>,
    route: Option<String>,
    props: Props<T, C>
}


/// Properties for ActiveWrapper.
#[derive(Properties)]
pub struct Props<T: for<'de> YewRouterState<'de>, C: Activatable> {
    children: ChildrenWithProps<C, ActiveWrapper<T, C>>
}

impl <T, C> Debug for Props<T,C>
where
    T: for<'de> YewRouterState<'de>,
    C: Activatable
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_struct("Props")
            .field("children", &"ChildrenWithProps<_, ActiveWrapper<_, _>".to_owned())
            .finish()
    }
}

/// Message type for ActiveWrapper
#[derive(Debug)]
pub enum Msg<T: for<'de> YewRouterState<'de>> {
    /// Message indicating that the route has changed
    RouteUpdated(RouteInfo<T>)
}

impl <T, C> Component for ActiveWrapper<T, C>
where
    T: for<'de> YewRouterState<'de>,
    C: Activatable
{
    type Message = Msg<T>;
    type Properties = Props<T, C>;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route_info| Msg::RouteUpdated(route_info));
        ActiveWrapper {
            router_bridge: RouteAgentBridge::new(callback),
            route: None,
            props
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.router_bridge.send(RouteRequest::GetCurrentRoute);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteUpdated(route_info) => self.route = Some(route_info.route)
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl <T, C> Renderable<ActiveWrapper<T, C>> for ActiveWrapper<T, C>
    where
        T: for<'de> YewRouterState<'de>,
        C: Activatable
{
    fn view(&self) -> Html<Self> {
        self.props.children.iter()
            .map(|mut child| {
                if let Some(route_string) = &self.route  {
                    child.props = C::set_active(child.props, &route_string)
                }
                child
            } )
            .collect()
    }
}
