use super::{AsEntity, Entity, Point, SketchTarget};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_LINE_SEGMENT},
    element::AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineSegment<T: SketchTarget> {
    pub point_a: Entity<Point<T>>,
    pub point_b: Entity<Point<T>>,
}

impl<T: SketchTarget> LineSegment<T> {
    pub fn new(point_a: Entity<Point<T>>, point_b: Entity<Point<T>>) -> Self {
        Self { point_a, point_b }
    }
}

impl<T: SketchTarget> AsEntity for LineSegment<T> {
    type SketchedOn = T;

    fn type_(&self) -> i32 {
        SLVS_E_LINE_SEGMENT as _
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
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
