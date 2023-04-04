use std::rc::Weak;

use super::{group::Group, Handle};

pub struct Param {
    h: u32,
    group: Weak<Group>,
    val: f64,
}

impl Default for Param {
    fn default() -> Self {
        Self {
            h: 0,
            group: Weak::new(),
            val: 0.0,
        }
    }
}

impl Param {
    pub fn new(group: &Weak<Group>, val: f64) -> Self {
        Self {
            group: Weak::clone(group),
            val,
            ..Default::default()
        }
    }
}

impl Handle for Param {
    fn get_handle(&self) -> u32 {
        self.h
    }

    fn set_handle(&mut self, h: u32) {
        self.h = h;
    }
}
