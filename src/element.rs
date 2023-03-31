pub mod group;
pub mod param;

use std::ops::RangeFrom;

use self::group::Group;

pub struct Elements<T> {
    list: Vec<T>,
    handle_gen: RangeFrom<u32>,
}

impl<T> Elements<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            handle_gen: (1..), // handle 0 is reserved for internal use, start from 1
        }
    }
}

impl Elements<group::Group> {
    pub fn add(&mut self) {
        self.list
            .push(group::Group::new(self.handle_gen.next().unwrap()));
    }
}

impl Elements<param::Param> {
    pub fn add(&mut self, group: Group, val: f64) {
        self.list.push(param::Param::new(
            self.handle_gen.next().unwrap(),
            group.into(),
            val,
        ))
    }
}
