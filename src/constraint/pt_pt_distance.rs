use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_PT_DISTANCE},
    element::AsHandle,
    entity::{Entity, SomePoint},
};

use super::AsConstraintData;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtPtDistance {
    point_a: Entity<SomePoint>,
    point_b: Entity<SomePoint>,
    distance: f64,
}

impl PtPtDistance {
    pub fn new(point_a: Entity<SomePoint>, point_b: Entity<SomePoint>, distance: f64) -> Self {
        Self {
            point_a,
            point_b,
            distance,
        }
    }
}

impl AsConstraintData for PtPtDistance {
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
