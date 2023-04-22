use std::marker::PhantomData;

use crate::{
    bindings::Slvs_hEntity,
    element::{AsHandle, Target},
};

mod point;
pub use point::{Coords, Point};
mod normal;
pub use normal::{Normal, NormalDef};
mod distance;
pub use distance::Distance;
mod workplane;
pub use workplane::Workplane;
mod line_segment;
pub use line_segment::LineSegment;
mod cubic;
pub use cubic::Cubic;
mod circle;
pub use circle::Circle;
mod arc_of_circle;
pub use arc_of_circle::ArcOfCircle;

////////////////////////////////////////////////////////////////////////////////
// Entity of a specific type
////////////////////////////////////////////////////////////////////////////////

pub trait AsEntity {
    type Sketch: Target + ?Sized;

    fn type_(&self) -> i32;
    fn points(&self) -> Option<Vec<Slvs_hEntity>>;
    fn normal(&self) -> Option<Slvs_hEntity>;
    fn distance(&self) -> Option<Slvs_hEntity>;
    fn param_vals(&self) -> Option<Vec<f64>>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Entity<T: AsEntity + ?Sized> {
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
