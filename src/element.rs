pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

use std::{
    ops::RangeFrom,
    rc::{Rc, Weak},
};
// pub trait Handle {
//     fn get_handle(&self) -> u32;
//     fn set_handle(&mut self, h: u32);
// }

pub(crate) struct Elements<T> {
    list: Vec<Rc<T>>,
    h_gen: RangeFrom<u32>,
}

impl<T> Elements<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            h_gen: (1..), // handle 0 is reserved for internal use, start from 1
        }
    }

    pub fn add(&mut self, element: T) -> Weak<T> {
        // element.set_handle(self.h_gen.next().unwrap());

        let element_ref = Rc::from(element);
        self.list.push(Rc::clone(&element_ref));
        Rc::downgrade(&element_ref)
    }
}
