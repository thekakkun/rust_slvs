use std::marker::PhantomData;

use crate::{
    bindings::Slvs_hEntity,
    element::{AsHandle},
};

mod pt_pt_distance;
pub use pt_pt_distance::PtPtDistance;
mod pt_line_distance;
pub use pt_line_distance::PtLineDistance;

pub trait AsConstraintData {
    fn type_(&self) -> i32;
    fn val(&self) -> Option<f64> {
        None
    }
    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }
    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }
    fn others(&self) -> [bool; 2] {
        [false, false]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraint<T: AsConstraintData> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsConstraintData> Constraint<T> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsConstraintData> AsHandle for Constraint<T> {
    fn as_handle(&self) -> u32 {
        self.handle
    }
}
