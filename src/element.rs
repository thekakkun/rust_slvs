use std::cell::{Ref, RefCell};

use crate::binding;

pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

pub(crate) type Elements<T> = RefCell<Vec<T>>;

pub(crate) trait PushLast<T> {
    fn push_last(&self, element: T) -> Ref<T>;
}

impl<T> PushLast<T> for Elements<T> {
    fn push_last(&self, element: T) -> Ref<T> {
        let mut vec = self.borrow_mut();
        vec.push(element);
        Ref::map(self.borrow(), |x| x.last().unwrap())
    }
}

// pub(crate) struct Elements<T>(pub(crate) Vec<T>);

// impl<T> Elements<T> {
//     fn new() -> Self {
//         Elements(Vec::new())
//     }

//     pub(crate) fn add(&mut self, element: T) -> &T {
//         self.0.push(element);
//         self.0.last().unwrap()
//     }
// }

// impl<T> Default for Elements<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Handle {
    Group(binding::Slvs_hGroup),
    Param(binding::Slvs_hParam),
    Entity(binding::Slvs_hEntity),
    Constraint(binding::Slvs_hConstraint),

}

impl From<Handle> for u32 {
    fn from(value: Handle) -> Self {
        match value {
            Handle::Group(h) | Handle::Param(h) | Handle::Entity(h) | Handle::Constraint(h) => h,
        }
    }
}