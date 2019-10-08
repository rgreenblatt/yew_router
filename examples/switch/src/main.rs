fn main() {
    println!("Hello, world!");
}
use yew_router::Switch;

#[derive(Switch)]
pub enum AppRoute {
    #[to = "/some/route"]
    SomeRoute
}