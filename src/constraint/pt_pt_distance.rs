use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_PT_PT_DISTANCE},
    element::AsHandle,
    entity::{AsPoint, Entity, Workplane},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    point_a: Entity<PA>,
    point_b: Entity<PB>,
    distance: f64,
    workplane: Option<Entity<Workplane>>,
}

impl<PA, PB> PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(
        point_a: Entity<PA>,
        point_b: Entity<PB>,
        distance: f64,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            point_a,
            point_b,
            distance,
            workplane,
        }
    }
}

impl<PA, PB> AsConstraintData for PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_PT_PT_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.as_handle())
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
    }
}

impl<PA, PB> From<Slvs_Constraint> for PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            point_a: Entity::new(value.ptA),
            point_b: Entity::new(value.ptB),
            distance: value.valA,
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
