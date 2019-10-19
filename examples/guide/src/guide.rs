use crate::{
    markdown_window::MarkdownWindow,
    page::{Page, PageProps},
};
use yew::{html::ChildrenWithProps, prelude::*, Properties};
use yew_router::{agent::RouteRequest::GetCurrentRoute, matcher::RouteMatcher, prelude::*};

pub struct Guide {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    route: Option<Route>,
    props: GuideProps,
}

#[derive(Properties)]
pub struct GuideProps {
    children: ChildrenWithProps<Page, Guide>,
}

pub enum Msg {
    UpdateRoute(Route),
}

impl Component for Guide {
    type Message = Msg;
    type Properties = GuideProps;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::UpdateRoute);
        let router_agent = RouteAgent::bridge(callback);
        Guide {
            router_agent,
            route: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.router_agent.send(GetCurrentRoute);
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute(route) => {
                self.route = Some(route);
            }
        }
        true
    }
}

impl Renderable<Guide> for Guide {
    fn view(&self) -> Html<Guide> {
        if let Some(route) = &self.route {
            let active_markdown_uri: Option<String> = self
                .props
                .children
                .iter()
                .filter_map(|child| {
                    if child.props.page_url == route.to_string() {
                        Some(child.props.uri)
                    } else {
                        None
                    }
                })
                .next();
            log::debug!("active uri: {:?}", active_markdown_uri);

            let mut list_items = self.props.children.iter().map(|child| {
                let x = render_page_list_item(child.props, route);
                if let yew::virtual_dom::VNode::VTag(x) = &x {
                    log::debug!("{:?}", x.attributes);
                }
                x
            });

            return html! {
                <div style="display: flex; overflow-y: hidden; height: 100%">
                    <div style="min-width: 280px; border-right: 2px solid black; overflow-y: auto">
                        <ul style="list-style: none; padding: 0; margin: 0">
                            {for list_items}
                        </ul>
                    </div>
                    <div style="overflow-y: auto; padding-left: 6px">
                    {
                        html !{
                            <MarkdownWindow uri=active_markdown_uri />
                        }
                    }
                    </div>
                </div>
            };
        } else {
            return html! {};
        }
    }
}

fn render_page_list_item(props: PageProps, route: &Route) -> Html<Guide> {
    let pm: RouteMatcher = RouteMatcher::try_from(&props.page_url).unwrap();
    if pm.capture_route_into_map(&route.to_string()).is_ok() {
        log::debug!("Found an active");
        return html! {
            <li style="padding-left: 4px; padding-right: 4px; padding-top: 6px; padding-bottom: 6px; background-color: lightgray;">
                <RouterLink link=props.page_url text={props.title} />
            </li>
        };
    } else {
        return html! {
            <li style="padding-left: 4px; padding-right: 4px; padding-top: 6px; padding-bottom: 6px; background-color: white;">
                <RouterLink link=props.page_url text={props.title} />
            </li>
        };
    }
}
