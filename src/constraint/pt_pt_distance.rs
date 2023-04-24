use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_PT_DISTANCE},
    element::AsHandle,
    entity::{AsEntityData, AsPoint, Entity},
};

use super::AsConstraintData;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtPtDistance<A, B>
where
    A: AsPoint + AsEntityData,
    B: AsPoint + AsEntityData,
{
    point_a: Entity<A>,
    point_b: Entity<B>,
    distance: f64,
}

impl<A, B> PtPtDistance<A, B>
where
    A: AsPoint + AsEntityData,
    B: AsPoint + AsEntityData,
{
    pub fn new(point_a: Entity<A>, point_b: Entity<B>, distance: f64) -> Self {
        Self {
            point_a,
            point_b,
            distance,
        }
    }
}

impl<A, B> AsConstraintData for PtPtDistance<A, B>
where
    A: AsPoint + AsEntityData,
    B: AsPoint + AsEntityData,
{
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
