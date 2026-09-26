use std::{
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hash},
};

use yew::{Callback, Html, UseStateHandle, hook, html, use_state, virtual_dom::Key};

use crate::use_mutator;

pub struct MapState<M, K, V> {
    pub state: UseStateHandle<M>,
    pub set: Callback<(K, V)>,
    pub delete: Callback<K>,
}

impl<M, K, V> MapState<M, K, V>
where
    M: MapLike<K, V>,
    K: Clone + 'static,
    V: Clone + 'static,
{
    /// Maps the data to a list of html elements and automatically keys them
    ///
    /// Identical to `.iter().map(|(k, v, c)| html!(<key={k.clone()}>{ f((k, v, c)) }</>))`
    pub fn keyed_list<'a, F>(&'a self, f: F) -> impl Iterator<Item = Html>
    where
        F: Fn((&K, &V, MapElementCallbacks<K, V>)) -> Html + 'a,
        K: Into<Key>,
    {
        self.iter()
            .map(move |(k, v, c)| html!(<key={k.clone()}>{ f((k, v, c)) }</>))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V, MapElementCallbacks<K, V>)> {
        self.state.iter().map(move |(k, v)| {
            let update_key = k.clone();
            let delete_key = k.clone();
            let callbacks = MapElementCallbacks {
                update: self.set.reform(move |x| (update_key.clone(), x)),
                remove: self.delete.reform(move |()| delete_key.clone()),
                insert: self.set.clone(),
            };
            (k, v, callbacks)
        })
    }
}

pub struct MapElementCallbacks<K, V> {
    pub update: Callback<V>,
    pub remove: Callback<()>,
    pub insert: Callback<(K, V)>,
}

pub trait MapLike<K, V> {
    fn remove(&mut self, key: K);
    fn set(&mut self, key: K, element: V);
    fn iter(&self) -> impl Iterator<Item = (&K, &V)>
    where
        K: 'static,
        V: 'static;
}

#[hook]
pub fn use_map_state<M, K, V, F>(init_fn: F) -> MapState<M, K, V>
where
    K: Clone + 'static,
    V: Clone + 'static,
    M: Clone + PartialEq + MapLike<K, V> + 'static,
    F: FnOnce() -> M,
{
    let state = use_state(init_fn);
    let set = use_mutator(&state, |state, (key, element)| state.set(key, element));
    let delete = use_mutator(&state, MapLike::remove);

    MapState { state, set, delete }
}

impl<K: Ord, V> MapLike<K, V> for BTreeMap<K, V> {
    fn remove(&mut self, key: K) {
        self.remove(&key);
    }

    fn set(&mut self, key: K, element: V) {
        self.insert(key, element);
    }

    fn iter(&self) -> impl Iterator<Item = (&K, &V)>
    where
        K: 'static,
        V: 'static,
    {
        self.iter()
    }
}

impl<K: Eq + Hash, V, B: BuildHasher> MapLike<K, V> for HashMap<K, V, B> {
    fn remove(&mut self, key: K) {
        self.remove(&key);
    }

    fn set(&mut self, key: K, element: V) {
        self.insert(key, element);
    }

    fn iter(&self) -> impl Iterator<Item = (&K, &V)>
    where
        K: 'static,
        V: 'static,
    {
        self.iter()
    }
}
