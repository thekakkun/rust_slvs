use std::marker::PhantomData;

use crate::{bindings::Slvs_hEntity, element::AsHandle};

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

pub trait AsEntityData {
    fn type_(&self) -> i32;
    fn workplane(&self) -> Option<Slvs_hEntity>;

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }
    fn normal(&self) -> Option<Slvs_hEntity> {
        None
    }
    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }
    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}

pub trait AsPoint: AsEntityData {}
pub trait AsLineSegment: AsEntityData {}
pub trait AsArc: AsEntityData {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Entity<T: AsEntityData> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsEntityData> Entity<T> {
    pub(super) fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsEntityData> AsHandle for Entity<T> {
    fn as_handle(&self) -> u32 {
        self.handle
    }
}
