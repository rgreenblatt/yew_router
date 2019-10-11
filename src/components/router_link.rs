//! A component wrapping an `<a>` tag that changes the route.
use crate::agent::{RouteRequest, RouteAgentDispatcher};
use crate::route::{Route};
use yew::prelude::*;

use super::Msg;
use super::Props;

/// An anchor tag Component that when clicked, will navigate to the provided route.
#[derive(Debug)]
pub struct RouterLink {
    router: RouteAgentDispatcher<()>,
    props: Props,
}

impl Component for RouterLink {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        RouterLink { router, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                let route = Route {
                    route: self.props.link.clone(),
                    state: self.props.state,
                };
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl Renderable<RouterLink> for RouterLink {
    fn view(&self) -> Html<RouterLink> {
        use stdweb::web::event::IEvent;
        let target: &str = &self.props.link;

        html! {
            <a
                class=self.props.classes.clone(),
                onclick=|event | {
                    event.prevent_default();
                    Msg::Clicked
                },
                disabled=self.props.disabled,
                href=target,
            >
                {&self.props.text}
            </a>
        }
    }
}
