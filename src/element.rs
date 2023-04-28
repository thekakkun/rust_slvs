use std::fmt::Debug;

use crate::{
    bindings::{Slvs_Constraint, Slvs_Entity, Slvs_Param},
    group::Group,
};

pub trait AsHandle: Debug {
    fn handle(&self) -> u32;
}

pub trait TypeInfo: Debug {
    fn type_of() -> String;
}

////////////////////////////////////////////////////////////////////////////////
// Storing Slvs_X in sys
////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub(super) struct SlvsElements {
    pub(super) groups: SlvsElementList<Group>,
    pub(super) params: SlvsElementList<Slvs_Param>,
    pub(super) entities: SlvsElementList<Slvs_Entity>,
    pub(super) constraints: SlvsElementList<Slvs_Constraint>,
}

pub(super) struct SlvsElementList<T> {
    pub(super) list: Vec<T>,
    pub(super) next_h: u32,
}

impl<T> SlvsElementList<T> {
    pub(super) fn new() -> Self {
        Self {
            list: Vec::new(),
            next_h: 1,
        }
    }

    pub(super) fn get_next_h(&mut self) -> u32 {
        let current_h = self.next_h;
        self.next_h += 1;

        current_h
    }
}

impl<T> Default for SlvsElementList<T> {
    fn default() -> Self {
        Self::new()
    }
}
