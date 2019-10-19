#![recursion_limit = "1024"]
mod a_component;
mod b_component;
mod c_component;

use yew::prelude::*;

use yew_router::prelude::*;
use yew_router::Switch;

use crate::a_component::AModel;
use crate::b_component::{BModel, BRoute};
use crate::c_component::CModel;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::initialize();
    web_logger::init();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

#[derive(Debug, Switch)]
pub enum AppRoute {
    #[to = "/a{*:inner}"]
    A(ARoute),
    #[to = "/b{*:inner}"]
    B(BRoute),
    #[to = "/c"]
    C,
    #[to = "/e/{string}"]
    E(String),
}

#[derive(Debug, Switch, PartialEq, Clone)]
pub enum ARoute {
    /// Match "/c" after "/a" ("/a/c")
    #[to = "/c"]
    C,
    // Because it is impossible to specify an Optional nested route:
    // Still accept the route, when matching, but consider it invalid.
    // This is effectively the same as wrapping the ARoute in Option, but doesn't run afoul of the current routing syntax.
    #[to = "{*:rest}"]
    None(String), // TODO, make this work on empty variants
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <RouterButton: text=String::from("Go to A"), link="/a", />
                    <RouterLink: text=String::from("Go to B"), link="/b/#", />
                    <RouterButton: text=String::from("Go to C"), link="/c", />
                    <RouterButton: text=String::from("Go to A/C"), link="/a/c", />
                    <RouterButton: text=String::from("Go to E (hello there)"), link="/e/there", />
                    <RouterButton: text=String::from("Go to E (hello world)"), link="/e/world", />
                    <RouterButton: text=String::from("Go to bad path"), link="/a_bad_path", />
                </nav>
                <div>
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: Option<AppRoute>| {
                            match switch {
                                Some(AppRoute::A(route)) => html!{<AModel route = route />},
                                Some(AppRoute::B(route)) => {
                                    let route: b_component::Props = route.into();
                                    html!{<BModel with route/>}
                                },
                                Some(AppRoute::C) => html!{<CModel />},
                                Some(AppRoute::E(string)) => html!{format!("hello {}", string)},
                                None => html!{"404"}
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}
