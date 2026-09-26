use std::{
    collections::{BTreeSet, HashSet},
    hash::{BuildHasher, Hash},
};

use yew::{Callback, Html, UseStateHandle, hook, html, use_state};

use crate::use_mutator;

pub struct SetState<M, V> {
    pub state: UseStateHandle<M>,
    pub set: Callback<V>,
    pub delete: Callback<V>,
}

impl<M, V> SetState<M, V>
where
    M: SetLike<V>,
    V: Clone + 'static,
{
    /// Sets the data to a list of html elements and automatically keys them
    ///
    /// Identical to `.iter().map(|(k, v, c)| html!(<key={k.clone()}>{ f((k, v, c)) }</>))`
    pub fn keyed_list<'a, F>(&'a self, f: F) -> impl Iterator<Item = Html>
    where
        F: Fn((&V, SetElementCallbacks<V>)) -> Html + 'a,
    {
        self.iter()
            .enumerate()
            .map(move |(k, (v, c))| html!(<key={k}>{ f((v, c)) }</>))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&V, SetElementCallbacks<V>)> {
        self.state.iter().map(move |v| {
            let delete_key = v.clone();
            let callbacks = SetElementCallbacks {
                remove: self.delete.reform(move |()| delete_key.clone()),
                insert: self.set.clone(),
            };
            (v, callbacks)
        })
    }
}

pub struct SetElementCallbacks<V> {
    pub remove: Callback<()>,
    pub insert: Callback<V>,
}

pub trait SetLike<V> {
    fn remove(&mut self, element: V);
    fn set(&mut self, element: V);
    fn iter(&self) -> impl Iterator<Item = &V>
    where
        V: 'static;
}

#[hook]
pub fn use_set_state<M, V, F>(init_fn: F) -> SetState<M, V>
where
    V: Clone + 'static,
    M: Clone + PartialEq + SetLike<V> + 'static,
    F: FnOnce() -> M,
{
    let state = use_state(init_fn);
    let set = use_mutator(&state, SetLike::set);
    let delete = use_mutator(&state, SetLike::remove);

    SetState { state, set, delete }
}

impl<V: Ord> SetLike<V> for BTreeSet<V> {
    fn remove(&mut self, element: V) {
        self.remove(&element);
    }

    fn set(&mut self, element: V) {
        self.insert(element);
    }

    fn iter(&self) -> impl Iterator<Item = &V>
    where
        V: 'static,
    {
        self.iter()
    }
}

impl<V: Eq + Hash, B: BuildHasher> SetLike<V> for HashSet<V, B> {
    fn remove(&mut self, element: V) {
        self.remove(&element);
    }

    fn set(&mut self, element: V) {
        self.insert(element);
    }

    fn iter(&self) -> impl Iterator<Item = &V>
    where
        V: 'static,
    {
        self.iter()
    }
}
