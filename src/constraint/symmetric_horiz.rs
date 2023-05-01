use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_SYMMETRIC_HORIZ},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct SymmetricHoriz<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    workplane: Entity<Workplane>,
    point_a: Entity<PA>,
    point_b: Entity<PB>,
}

impl<PA, PB> SymmetricHoriz<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(workplane: Entity<Workplane>, point_a: Entity<PA>, point_b: Entity<PB>) -> Self {
        Self {
            workplane,
            point_a,
            point_b,
        }
    }
}

impl<PA, PB> AsConstraintData for SymmetricHoriz<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_SYMMETRIC_HORIZ as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<PA, PB> TypeInfo for SymmetricHoriz<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_of() -> String {
        format!("SymmetricHoriz < {}, {}, >", PA::type_of(), PB::type_of())
    }
}

impl<PA, PB> From<Slvs_Constraint> for SymmetricHoriz<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            workplane: Entity::new(value.wrkpl),
            point_a: Entity::new(value.ptA),
            point_b: Entity::new(value.ptB),
        }
    }
}