use yew::{Callback, Html, UseStateHandle, hook, html, use_state};

use crate::use_mutator;

pub struct ListState<L, T> {
    pub state: UseStateHandle<L>,
    pub update: Callback<(usize, T)>,
    pub delete: Callback<usize>,
    pub push: Callback<T>,
}

impl<L: VectorLike<T>, T> ListState<L, T> {
    pub fn push_default(&self) -> Callback<()>
    where
        T: Default + 'static,
    {
        self.push.reform(|()| T::default())
    }

    /// Maps the data to a list of html elements and automatically keys them
    ///
    /// Identical to `.iter().map(|(k, v, c)| html!(<key={k.clone()}>{ f((v, c)) }</>))`
    pub fn keyed_list<'a, F>(&'a self, f: F) -> impl Iterator<Item = Html>
    where
        F: Fn((&T, ListElementCallbacks<T>)) -> Html + 'a,
        T: 'static,
    {
        self.iter()
            .map(move |(k, v, c)| html!(<key={k}>{ f((v, c)) }</>))
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &T, ListElementCallbacks<T>)>
    where
        T: 'static,
    {
        self.state.iter().enumerate().map(move |(k, v)| {
            let callbacks = ListElementCallbacks {
                update: self.update.reform(move |x| (k, x)),
                remove: self.delete.reform(move |()| k),
                push: self.push.clone(),
            };
            (k, v, callbacks)
        })
    }
}

pub struct ListElementCallbacks<T> {
    pub update: Callback<T>,
    pub remove: Callback<()>,
    pub push: Callback<T>,
}

pub trait VectorLike<T> {
    fn get(&self, index: usize) -> &T;
    fn remove(&mut self, index: usize);
    fn push(&mut self, element: T);
    fn update(&mut self, index: usize, element: T);
    fn iter(&self) -> impl Iterator<Item = &T>
    where
        T: 'static;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> VectorLike<T> for Vec<T>
where
    T: 'static,
{
    fn get(&self, index: usize) -> &T {
        &self[index]
    }

    fn remove(&mut self, index: usize) {
        self.remove(index);
    }

    fn push(&mut self, element: T) {
        self.push(element);
    }

    fn update(&mut self, index: usize, element: T) {
        self[index] = element;
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        <[T]>::iter(self)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

#[hook]
pub fn use_list_state<L, T, F>(init_fn: F) -> ListState<L, T>
where
    T: Clone + 'static,
    L: Clone + PartialEq + VectorLike<T> + 'static,
    F: FnOnce() -> L,
{
    let state = use_state(init_fn);
    let update = use_mutator(&state, |state, (idx, t)| {
        if state.len() > idx {
            state.update(idx, t);
        }
    });

    let delete = use_mutator(&state, VectorLike::remove);
    let push = use_mutator(&state, VectorLike::push);

    ListState {
        state,
        update,
        delete,
        push,
    }
}
