use std::marker::PhantomData;

use crate::{bindings::Slvs_hEntity, element::AsHandle};

pub mod point;
pub use point::Point;
pub mod normal;
pub use normal::Normal;
pub mod distance;
pub use distance::Distance;
pub mod workplane;
pub use workplane::Workplane;
pub mod line_segment;
pub use line_segment::LineSegment;
pub mod cubic;
pub use cubic::Cubic;
pub mod circle;
pub use circle::Circle;
pub mod arc_of_circle;
pub use arc_of_circle::ArcOfCircle;

////////////////////////////////////////////////////////////////////////////////
// Entity of a specific type
////////////////////////////////////////////////////////////////////////////////

pub trait AsEntity {
    type SketchedOn;

    fn type_(&self) -> i32;
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

trait SketchTarget {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OnWorkplane {}
impl SketchTarget for OnWorkplane {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreeIn3d {}
impl SketchTarget for FreeIn3d {}
