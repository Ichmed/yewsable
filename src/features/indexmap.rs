use std::hash::{BuildHasher, Hash};

use crate::{MapLike, set_state::SetLike};

impl<V: Eq + Hash, B: BuildHasher> SetLike<V> for indexmap::IndexSet<V, B> {
    fn remove(&mut self, element: V) {
        self.shift_remove(&element);
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

impl<K: Eq + Hash, V, B: BuildHasher> MapLike<K, V> for indexmap::IndexMap<K, V, B> {
    fn remove(&mut self, key: K) {
        self.shift_remove(&key);
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
