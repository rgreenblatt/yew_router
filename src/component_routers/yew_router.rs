//! Component that performs routing.

use yew::prelude::*;
use router::{Route, Router};
use yew::html::Component;
use router::Request as RouterRequest;

use yew::virtual_dom::VNode;
use yew::virtual_dom::VList;
use yew::agent::Transferable;


use yew_patterns::{Sender, Receiver};

use component_routers::ComponentConstructorAttempter;

pub enum Msg {
    SetRoute(Route<()>),
    SendRoutingFailure,
    ReceiveRoutingFailure,
    NoOp
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoutingFailedMsg;

impl Transferable for RoutingFailedMsg {}

enum FailedChannel {
    Sender(Sender<RoutingFailedMsg>),
    Receiver(Receiver<RoutingFailedMsg>)
}


#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub routes: Vec<ComponentConstructorAttempter>,
    pub page_not_found: Option<DefaultPage>
}

pub struct YewRouter {
    link: ComponentLink<YewRouter>,
    router: Box<Bridge<Router<()>>>,
    route: Route<()>,
    routing_succeeded: bool,
    channel: FailedChannel,
    routes: Vec<ComponentConstructorAttempter>,
    page_not_found: Option<DefaultPage>
}



#[derive(Clone)]
pub struct DefaultPage(pub fn(route: &Route<()>) -> VNode<YewRouter>);

impl PartialEq for DefaultPage {
    fn eq(&self, other: &DefaultPage) -> bool {
        // compare pointers // TODO investigate if this works?
        self.0 as *const () == other.0 as *const ()
    }
}
impl Default for DefaultPage {
    fn default() -> Self {
        fn default_page_impl(_route: &Route<()>) -> VNode<YewRouter> {
            VNode::VList(VList::new())
        }
        DefaultPage(default_page_impl)
    }
}

impl Component for YewRouter {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let callback = link.send_back(|route: Route<()>| Msg::SetRoute(route));
        let router = Router::bridge(callback);
        // TODO Not sure if this is technically correct. This should be sent _after_ the component has been created.
        router.send(RouterRequest::GetCurrentRoute);


        // If the component is created with a page_not_found page,
        // then it needs to be able to receive messages telling it that another router failed.
        let channel = if let Some(_) = props.page_not_found {
            let callback = link.send_back(|_| Msg::ReceiveRoutingFailure);
            FailedChannel::Receiver(Receiver::new(callback))
        } else {
            let callback = link.send_back(|_| Msg::NoOp);
            FailedChannel::Sender(Sender::new(callback))
        };

        YewRouter {
            link,
            router,
            route: Route::default(), // Empty route, may or may not match any possible routes
            routing_succeeded: false, // TODO this doesn't adequately capture the meaning that I want. Consider an enum around the Route instead
            channel,
            routes: props.routes,
            page_not_found: props.page_not_found
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetRoute(route) => {
                self.route = route;
                if self.can_resolve_child() {
                    self.routing_succeeded = true;
                } else {
                    self.routing_succeeded = false;
                    self.update(Msg::SendRoutingFailure);
                }
                true
            }
            Msg::SendRoutingFailure => {
                if let FailedChannel::Sender(ref sender) = self.channel {
                    sender.send(RoutingFailedMsg)
                }
                false
            }
            Msg::ReceiveRoutingFailure => {
//                self.route = None;
                self.routing_succeeded = false;
                false
            }
            Msg::NoOp => false
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {

        // TODO don't recreate unless absolutely necessary.
        let channel = if let Some(_) = props.page_not_found {
            let callback = self.link.send_back(|_| Msg::ReceiveRoutingFailure);
            FailedChannel::Receiver(Receiver::new(callback))
        } else {
            let callback = self.link.send_back(|_| Msg::NoOp);
            FailedChannel::Sender(Sender::new(callback))
        };

        self.channel = channel;

        self.routes = props.routes;
        self.page_not_found = props.page_not_found;
        true
    }
}

impl YewRouter {
    /// Determines which child to render based on the current route
    /// If none of the sub components can be rendered, return None.
    fn resolve_child(&self) -> Option<VNode<YewRouter>> {
        if self.routing_succeeded {
            for resolver in &self.routes {
                if let Some(child) = (resolver.0)(&self.route) {
                   return Some(child)
                }
            }
        }
        return None
    }
    fn can_resolve_child(&self) -> bool {
        for resolver in &self.routes {
            if let Some(_child) = (resolver.0)(&self.route) {
               return true
            }
        }
        false
    }
}


impl Renderable<YewRouter> for YewRouter {
    fn view(&self) -> Html<YewRouter> {

        // TODO this resolve child sort of runs twice, which ideally could be avoided
        if let Some(child) = self.resolve_child() {
            child
        }else {
            if let Some(ref page_not_found) = self.page_not_found {
                (page_not_found.0)(&self.route)
            } else {
                VNode::VList(VList::new()) // empty - no matched route
            }
        }

//        if let Some(child) = self.resolve_child() {
//            child
//        } else {
//            if let Some(ref page_not_found) = self.page_not_found {
//                (page_not_found.0)(&self.route)
//            } else {
//                VNode::VList(VList::new()) // empty - no matched route
//            }
//        }
    }
}



/// Turns the provided component type name into a wrapped function that will create the component.
#[macro_export]
macro_rules! routes {
    ( $( $x:tt ),* ) => {
        vec![$(<($x)>::ROUTING_CONSTRUCTOR_ATTEMPTER )*]
    };
}
