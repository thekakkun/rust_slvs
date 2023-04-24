use super::{AsEntityData, Entity, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_LINE_SEGMENT},
    element::{AsHandle, AsTarget},
};

pub trait AsLineSegment {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineSegment<T: AsTarget> {
    pub point_a: Entity<Point<T>>,
    pub point_b: Entity<Point<T>>,
}

impl<T: AsTarget> LineSegment<T> {
    pub fn new(point_a: Entity<Point<T>>, point_b: Entity<Point<T>>) -> Self {
        Self { point_a, point_b }
    }
}

impl<T: AsTarget> AsLineSegment for LineSegment<T> {}

impl<T: AsTarget> AsEntityData for LineSegment<T> {
    type Sketch = T;

    fn type_(&self) -> i32 {
        SLVS_E_LINE_SEGMENT as _
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
    }
}
