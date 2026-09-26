#[cfg(feature = "async")]
pub mod r#async;
mod features;
pub mod input;
pub mod list_state;
pub mod map_state;
pub mod set_state;

#[cfg(feature = "async")]
pub use r#async::*;
pub use features::*;
pub use list_state::*;
pub use map_state::*;

use gloo::{
    events::{EventListener, EventListenerOptions},
    utils::window,
};
use web_sys::{EventTarget, HtmlElement};
use yew::{Callback, NodeRef, UseStateHandle, hook, use_callback, use_effect_with, use_node_ref};

#[hook]
pub fn use_setter<T, IN>(state_handle: &UseStateHandle<T>, value: T) -> Callback<IN, ()>
where
    T: Clone + PartialEq + 'static,
    IN: 'static,
{
    use_callback(
        (state_handle.clone(), value),
        move |_: IN, (handle, value)| handle.set(value.clone()),
    )
}

#[hook]
pub fn use_event_listener<D, F>(deps: D, target: &EventTarget, event_name: &'static str, f: F)
where
    D: Clone + PartialEq + 'static,
    F: Fn(&web_sys::Event, &D) + 'static,
{
    use_effect_with((deps, target.clone()), move |(deps, target)| {
        let deps = deps.clone();
        let target = target.clone();
        let options = EventListenerOptions::enable_prevent_default();
        let listener =
            EventListener::new_with_options(&target, event_name, options, move |event| {
                f(event, &deps);
            });

        move || drop(listener)
    });
}

#[hook]
pub fn use_window_event<D, F>(deps: D, event_name: &'static str, f: F)
where
    D: Clone + PartialEq + 'static,
    F: Fn(&web_sys::Event, &D) + 'static,
{
    use_event_listener::<D, F>(deps, &window(), event_name, f);
}

#[hook]
pub fn use_focus() -> (NodeRef, Callback<()>) {
    let node_ref = use_node_ref();
    let focus_callback = use_callback(node_ref.clone(), |(), query_input| {
        query_input.cast::<HtmlElement>().unwrap().focus().unwrap();
    });
    (node_ref, focus_callback)
}

#[hook]
/// Shorthand for the common
///
/// ```
/// let new = (*state).clone();
/// // do something to new
/// state.set(new);
/// ```
///
/// pattern.
///
/// Note that the order of arguments is reversed compared to `use_callback`, this is to enable passing in
/// methods on `T` directly
pub fn use_mutator<T, F, IN, OUT>(handle: &UseStateHandle<T>, f: F) -> Callback<IN, ()>
where
    T: Clone + PartialEq + 'static,
    F: Fn(&mut T, IN) -> OUT + 'static,
    IN: 'static,
{
    use_callback(handle.clone(), move |x, handle| {
        let mut new = (**handle).clone();
        f(&mut new, x);
        handle.set(new);
    })
}
