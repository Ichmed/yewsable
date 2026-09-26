use iddqd::IdHashItem;

use crate::set_state::SetLike;

impl<V> SetLike<V> for iddqd::IdHashMap<V>
where
    V: IdHashItem,
{
    fn remove(&mut self, element: V) {
        self.remove(element.key());
    }

    fn set(&mut self, element: V) {
        self.insert_overwrite(element);
    }

    fn iter(&self) -> impl Iterator<Item = &V>
    where
        V: 'static,
    {
        self.iter()
    }
}
