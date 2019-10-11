//! A component wrapping a `<button>` tag that changes the route.
use crate::agent::{RouteRequest, RouteAgentDispatcher};
use crate::route::{Route};
use yew::prelude::*;

use super::Msg;
use super::Props;

/// Changes the route when clicked.
#[derive(Debug)]
pub struct RouterButton {
    router: RouteAgentDispatcher<()>,
    props: Props,
}

impl Component for RouterButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        RouterButton { router, props }
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

impl Renderable<RouterButton> for RouterButton {
    fn view(&self) -> Html<RouterButton> {
        html! {
            <button
                class=self.props.classes.clone(),
                onclick=|_| Msg::Clicked,
                disabled=self.props.disabled,
            >
                {&self.props.text}
            </button>
        }
    }
}
