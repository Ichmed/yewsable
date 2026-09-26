use yew::{Callback, hook, use_callback, use_effect, use_effect_with};

#[hook]
pub fn use_async_callback<IN, F, D>(deps: D, f: F) -> Callback<IN, ()>
where
    IN: 'static,
    F: AsyncFn(IN, &D) -> () + Clone + 'static,
    D: PartialEq + Clone + 'static,
{
    use_callback(deps, move |input, deps| {
        let b = deps.clone();
        let f = f.clone();
        wasm_bindgen_futures::spawn_local(Box::pin(async move { f(input, &b).await }));
    })
}

#[hook]
pub fn use_async_effect<F>(f: F)
where
    F: AsyncFn() -> () + Clone + 'static,
{
    use_effect(move || {
        let f = f;
        wasm_bindgen_futures::spawn_local(Box::pin(async move { f().await }));
    });
}

#[hook]
pub fn use_async_effect_with<F, D>(deps: D, f: F)
where
    F: AsyncFn(&D) -> () + Clone + 'static,
    D: PartialEq + Clone + 'static,
{
    use_effect_with(deps, move |deps| {
        let b = deps.clone();
        let f = f;
        wasm_bindgen_futures::spawn_local(Box::pin(async move { f(&b).await }));
    });
}
