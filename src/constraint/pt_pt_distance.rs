use super::AsConstraint;
use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_PT_DISTANCE},
    element::{AsHandle, Target},
    entity::{Entity, Point},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtPtDistance<T: Target> {
    point_a: Entity<Point<T>>,
    point_b: Entity<Point<T>>,
    distance: f64,
}

impl<T: Target> PtPtDistance<T> {
    pub fn new(point_a: Entity<Point<T>>, point_b: Entity<Point<T>>, distance: f64) -> Self {
        Self {
            point_a,
            point_b,
            distance,
        }
    }
}

impl<T: Target> AsConstraint for PtPtDistance<T> {
    type Apply = T;

    fn type_(&self) -> i32 {
        SLVS_C_PT_PT_DISTANCE as _
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }

    fn others(&self) -> [bool; 2] {
        [false, false]
    }
}
