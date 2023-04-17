use std::marker::PhantomData;

use crate::{bindings::Slvs_hEntity, AsHandle};

pub mod line_segment;
pub use line_segment::LineSegment;
pub mod point_in_3d;
pub use point_in_3d::PointIn3d;

////////////////////////////////////////////////////////////////////////////////
// Entity of a specific type
////////////////////////////////////////////////////////////////////////////////

pub trait AsEntity {
    fn type_(&self) -> i32;
    fn workplane(&self) -> Option<Slvs_hEntity>;
    fn points(&self) -> Option<Vec<Slvs_hEntity>>;
    fn normal(&self) -> Option<Slvs_hEntity>;
    fn distance(&self) -> Option<Slvs_hEntity>;
    fn param_vals(&self) -> Option<Vec<f64>>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Entity<T: AsEntity> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsEntity> Entity<T> {
    pub(super) fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsEntity> AsHandle for Entity<T> {
    fn as_handle(&self) -> u32 {
        self.handle
    }
}
