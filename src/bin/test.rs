use indexmap::IndexMap;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yewsable::{MapElementCallbacks, input::InputEventExt, use_map_state};

#[autoprops]
#[component]
fn Test() -> Html {
    let x = use_map_state(|| IndexMap::<AttrValue, AttrValue>::from([("Hi".into(), "Ho".into())]));

    x.iter()
        .map(
            |(
                name,
                value,
                MapElementCallbacks {
                    update,
                    remove,
                    insert,
                },
            )| {
                let onchange =
                    update.reform(|event: Event| event.input_value().unwrap_or_default().into());
                let new_name = value.clone();
                html!(
                    <div>
                        <h1>{ name }</h1>
                        <input value={value.clone()} {onchange} />
                        <button onclick={remove.reform(|_| ())}>{ "X" }</button>
                        <button onclick={insert.reform(move |_| (new_name.clone(), "".into()))}>{ "+" }</button>
                    </div>
                )
            },
        )
        .collect()
}

fn main() {
    yew::Renderer::<Test>::new().render();
}
