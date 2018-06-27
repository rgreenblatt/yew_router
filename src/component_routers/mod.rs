mod yew_router;

use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::virtual_dom::VComp;

use router::Route;

pub use self::yew_router::{YewRouter, DefaultPage, Props};



/// A trait that allows a component to be routed by a Yew router.
pub trait Routable: Component + Renderable<Self> {
    /// Try to construct the props used for creating a Component from route info.
    /// If None is returned, the router won't create the component.
    /// If Some(_) is returned, the router will create the component using the props
    /// and will stop trying to create other components.
    fn resolve_props(route: &Route<()>) -> Option<<Self as Component>::Properties>;

    /// This is a wrapped function pointer to a function that will try to create a component if the route matches.
    const ROUTING_CONSTRUCTOR_ATTEMPTER: ComponentConstructorAttempter = ComponentConstructorAttempter(try_construct_component_from_route::<Self>);
}

/// For a component that allows its props to be constructed from the Route,
/// this function will instansiate the component within the context of the YewRouter.
fn try_construct_component_from_route<T: Routable>(route: &Route<()>) -> Option<VNode<YewRouter>> {
    if let Some(props) = T::resolve_props(route) {
        let mut comp = VComp::lazy::<T>().1; // Creates a component
        comp.set_props(props); // The properties of the component _must_ be set
        return Some(VNode::VComp(comp))
    }

    return None
}

#[derive(Clone)]
pub struct ComponentConstructorAttempter(fn(route: &Route<()>) -> Option<VNode<YewRouter>>);

impl PartialEq for ComponentConstructorAttempter {
    fn eq(&self, other: &ComponentConstructorAttempter) -> bool {
        // compare pointers // TODO investigate if this works?
        self.0 as *const () == other.0 as *const ()
    }
}
