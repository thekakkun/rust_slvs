use super::AsConstraint;
use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_PT_DISTANCE},
    element::{AsHandle, Target},
    entity::{Entity, Point},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtPtDistance<T: Target> {
    val: f64,
    point_a: Entity<Point<T>>,
    point_b: Entity<Point<T>>,
}

impl<T: Target> PtPtDistance<T> {
    pub fn new(val: f64, point_a: Entity<Point<T>>, point_b: Entity<Point<T>>) -> Self {
        Self {
            val,
            point_a,
            point_b,
        }
    }
}

impl<T: Target> AsConstraint for PtPtDistance<T> {
    type Apply = T;

    fn type_(&self) -> i32 {
        SLVS_C_PT_PT_DISTANCE as _
    }

    fn val(&self) -> Option<f64> {
        Some(self.val)
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
