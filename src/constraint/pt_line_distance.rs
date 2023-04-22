use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_LINE_DISTANCE},
    element::{AsHandle, Target},
    entity::{Entity, LineSegment, Point},
};

use super::AsConstraint;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtLineDistance<T: Target> {
    point: Entity<Point<T>>,
    line: Entity<LineSegment<T>>,
    distance: f64,
}

impl<T: Target> PtLineDistance<T> {
    pub fn new(point: Entity<Point<T>>, line: Entity<LineSegment<T>>, distance: f64) -> Self {
        Self {
            point,
            line,
            distance,
        }
    }
}

impl<T: Target> AsConstraint for PtLineDistance<T> {
    type Apply = T;

    fn type_(&self) -> i32 {
        SLVS_C_PT_LINE_DISTANCE as _
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.as_handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }

    fn others(&self) -> [bool; 2] {
        [false, false]
    }
}
