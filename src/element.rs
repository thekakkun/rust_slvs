use std::fmt::Debug;

use crate::{
    bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    constraint::AsConstraint,
    entity::AsEntity,
};

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

#[derive(Debug, Default)]
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

impl Debug for dyn AsEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Entity: {{handle: {}, type: {}}}",
            self.handle(),
            self.type_name()
        )
    }
}

impl Debug for dyn AsConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Constraint: {{handle: {}, type: {}}}",
            self.handle(),
            self.type_name()
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// Wrapper for elements with handles
////////////////////////////////////////////////////////////////////////////////

pub trait AsElementIdentifier {
    fn handle(&self) -> u32;
    fn type_name(&self) -> String;
}

////////////////////////////////////////////////////////////////////////////////
// Group
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group(pub(super) u32);

impl AsElementIdentifier for Group {
    fn handle(&self) -> u32 {
        self.0
    }

    fn type_name(&self) -> String {
        "Group".to_string()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Sketch targets (OnWorkplane & In3d)
////////////////////////////////////////////////////////////////////////////////

pub trait AsTarget: Copy + Debug {
    fn type_() -> i32;
    fn as_vec(&self) -> Vec<f64>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_2D as _
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.0, self.1]
    }
}

impl From<Vec<f64>> for OnWorkplane {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_3D as _
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.0, self.1, self.2]
    }
}

impl From<Vec<f64>> for In3d {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1], value[2])
    }
}
