use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_PT_DISTANCE},
    element::AsHandle,
    entity::{AsEntityData, AsPoint, Entity},
};

use super::AsConstraintData;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtPtDistance<PA, PB>
where
    PA: AsPoint + AsEntityData,
    PB: AsPoint + AsEntityData,
{
    point_a: Entity<PA>,
    point_b: Entity<PB>,
    distance: f64,
}

impl<PA, PB> PtPtDistance<PA, PB>
where
    PA: AsPoint + AsEntityData,
    PB: AsPoint + AsEntityData,
{
    pub fn new(point_a: Entity<PA>, point_b: Entity<PB>, distance: f64) -> Self {
        Self {
            point_a,
            point_b,
            distance,
        }
    }
}

impl<PA, PB> AsConstraintData for PtPtDistance<PA, PB>
where
    PA: AsPoint + AsEntityData,
    PB: AsPoint + AsEntityData,
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
}
