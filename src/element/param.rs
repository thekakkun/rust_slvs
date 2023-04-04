use std::rc::{Rc, Weak};

use super::{group::Group, Elements};

pub(crate) struct Param {
    h: u32,
    group: Weak<Group>,
    val: f64,
}

// impl Handle for Param {
//     fn get_handle(&self) -> u32 {
//         self.h
//     }

//     fn set_handle(&mut self, h: u32) {
//         self.h = h;
//     }
// }


impl Elements<Param> {
    // pub fn add(&mut self, group: &Weak<Group>, val: f64) -> Weak<Param> {
    //     let new_param = Rc::new(Param {
    //         h: self.h_gen.next().unwrap(),
    //         group: Weak::clone(group),
    //         val,
    //     });

    //     self.list.push(Rc::clone(&new_param));
    //     Rc::downgrade(&new_param)
    // }
}
