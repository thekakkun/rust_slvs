use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_SYMMETRIC_VERT},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct SymmetricVert<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    workplane: Entity<Workplane>,
    point_a: Entity<PA>,
    point_b: Entity<PB>,
}

impl<PA, PB> SymmetricVert<PA, PB>
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

impl<PA, PB> AsConstraintData for SymmetricVert<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_SYMMETRIC_VERT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<PA, PB> TypeInfo for SymmetricVert<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_of() -> String {
        format!("SymmetricVert < {}, {}, >", PA::type_of(), PB::type_of())
    }
}

impl<PA, PB> From<Slvs_Constraint> for SymmetricVert<PA, PB>
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