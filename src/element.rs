use std::fmt::Debug;

use crate::{constraint::AsConstraint, entity::AsEntity, group::Group};

////////////////////////////////////////////////////////////////////////////////
// Storing Slvs_X in sys
////////////////////////////////////////////////////////////////////////////////

pub(super) struct SlvsElements<T> {
    pub(super) list: Vec<T>,
    pub(super) next_h: u32,
}

impl<T> SlvsElements<T> {
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

impl<T> Default for SlvsElements<T> {
    fn default() -> Self {
        Self::new()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Storing element identifiers in sys
////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct Elements {
    pub groups: Vec<Group>,
    pub entities: Vec<Box<dyn AsEntity>>,
    pub constraints: Vec<Box<dyn AsConstraint>>,
}

impl Elements {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            entities: Vec::new(),
            constraints: Vec::new(),
        }
    }
}

impl Clone for Box<dyn AsConstraint> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

pub trait AsHandle: Debug {
    fn handle(&self) -> u32;
}

pub trait TypeInfo: Debug {
    fn type_of() -> String;
}
