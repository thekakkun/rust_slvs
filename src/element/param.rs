use std::rc::Rc;

use super::group::Group;

pub struct Param {
    h: u32,
    group: Rc<Group>,
    val: f64,
}

impl Param {
    pub fn new(h: u32, group: Rc<Group>, val: f64) -> Self {
        Self { h, group, val }
    }
}
