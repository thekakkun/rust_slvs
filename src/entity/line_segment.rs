use super::{AsEntityData, Entity, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_LINE_SEGMENT},
    element::{AsHandle, Target},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineSegment<T: Target> {
    pub point_a: Entity<Point<T>>,
    pub point_b: Entity<Point<T>>,
}

impl<T: Target> LineSegment<T> {
    pub fn new(point_a: Entity<Point<T>>, point_b: Entity<Point<T>>) -> Self {
        Self { point_a, point_b }
    }
}

impl<T: Target> AsEntityData for LineSegment<T> {
    type Sketch = T;

    fn type_(&self) -> i32 {
        SLVS_E_LINE_SEGMENT as _
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
    }
}
