use std::ops::{Deref, DerefMut};

use yew::{Callback, UseStateHandle, hook, use_state};

use crate::use_mutator;

pub struct ListState<L, T> {
    pub state: UseStateHandle<L>,
    pub update: Callback<(usize, T)>,
    pub delete: Callback<usize>,
    pub push: Callback<T>,
}

impl<L, T> ListState<L, T> {
    pub fn push_default(&self) -> Callback<()>
    where
        T: Default + 'static,
    {
        self.push.reform(|()| T::default())
    }
}

pub trait VectorLike<T> {
    fn get(&self, index: usize) -> &T;
    fn remove(&mut self, index: usize);
    fn push(&mut self, element: T);
    fn update(&mut self, index: usize, element: T);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<X, T> VectorLike<T> for X
where
    X: BackedByVec<T>,
{
    fn get(&self, index: usize) -> &T {
        &self.as_backing_vec()[index]
    }

    fn remove(&mut self, index: usize) {
        self.as_backing_vec_mut().remove(index);
    }

    fn push(&mut self, element: T) {
        self.as_backing_vec_mut().push(element);
    }

    fn update(&mut self, index: usize, element: T) {
        self.as_backing_vec_mut()[index] = element;
    }

    fn len(&self) -> usize {
        self.as_backing_vec().len()
    }
}

pub trait BackedByVec<T> {
    fn as_backing_vec_mut(&mut self) -> &mut Vec<T>;
    fn as_backing_vec(&self) -> &Vec<T>;
}

impl<L, T> BackedByVec<T> for L
where
    L: DerefMut + Deref<Target = Vec<T>>,
{
    fn as_backing_vec_mut(&mut self) -> &mut Vec<T> {
        self
    }

    fn as_backing_vec(&self) -> &Vec<T> {
        self
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
