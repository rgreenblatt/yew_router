/// Generates a set of aliases to common structures within yew_router.
///
/// Because they should be the same across a given application,
/// its a handy way to make sure that every type that could be needed is generated.
#[macro_export]
macro_rules! router_aliases {
    ($StateT:ty) => {
        router_aliases!($StateT, stringify!($StateT));
    };
    ($StateT:ty, $StateName:expr) => {
        mod router_aliases {
            use $crate::matcher::RenderFn;

            #[doc = "Alias to [RouteInfo<"]
            #[doc = $StateName]
            #[doc = ">](route_info/struct.RouteInfo.html)."]
            pub type RouteInfo = $crate::route_info::RouteInfo<$StateT>;

            #[doc = "Alias to [RouteService<"]
            #[doc = $StateName]
            #[doc = ">](route_service/struct.RouteService.html)."]
            pub type RouteService = $crate::route_service::RouteService<$StateT>;

            #[cfg(feature="router_agent")]
            #[doc = "Alias to [RouteAgent<"]
            #[doc = $StateName]
            #[doc = ">](agent/struct.RouteAgent.html)."]
            pub type RouteAgent = $crate::agent::RouteAgent<$StateT>;

            #[cfg(feature="router_agent")]
            #[doc = "Alias to [RouteAgentBridge<"]
            #[doc = $StateName]
            #[doc = ">](agent/bridge/struct.RouteAgentBridge.html)`."]
            pub type RouteAgentBridge = $crate::agent::bridge::RouteAgentBridge<$StateT>;

            #[cfg(feature="router")]
            #[doc = "Alias to [Router<"]
            #[doc = $StateName]
            #[doc = ">](router_component/router/struct.Router.html)."]
            pub type Router = $crate::router_component::router::Router<$StateT>;

            #[cfg(feature="router")]
            #[doc = "Alias to [Route<"]
            #[doc = $StateName]
            #[doc = ">](router_component/route/struct.Route.html)."]
            pub type Route = $crate::router_component::route::Route<$StateT>;

            #[cfg(feature="router")]
            #[doc = "Alias to [Render<"]
            #[doc = $StateName]
            #[doc = ">](router_component/render/struct.Render.html)."]
            pub type Render = $crate::router_component::render::Render<$StateT>;

            #[cfg(feature="router")]
            #[doc = "Renders the provided closure in terms of a `Router<"]
            #[doc = $StateName]
            #[doc = ">`"]
            pub fn render(render: impl RenderFn<Router> + 'static) -> $crate::router_component::render::Render<$StateT> {
                $crate::router_component::render::render_s(render)
            }

            #[cfg(feature="router")]
            #[doc = "Creates a components using a Html block in terms of a `Router<"]
            #[doc = $StateName]
            #[doc = ">`"]
            pub fn component<T>() -> $crate::router_component::render::Render<$StateT>
            where
                T: yew::Component + yew::Renderable<T>,
                <T as yew::Component>::Properties: crate::matcher::FromCaptures,
            {
                $crate::router_component::render::component_s::<T, $StateT>()
            }
        }
    }
}

