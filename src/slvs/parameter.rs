use std::rc::Rc;

use super::bindings;

pub struct Param {
    h: bindings::Slvs_hParam,
    group: Rc<super::Group>,
    val: f64,
}

impl Param {
    pub fn new(h: bindings::Slvs_hParam, group: &Rc<super::Group>, val: f64) -> Self {
        Self {
            h,
            group: Rc::clone(group),
            val,
        }
    }
}
