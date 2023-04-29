use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_SYMMETRIC_LINE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct SymmetricLine<PA, PB, L>
where
    PA: AsPoint,
    PB: AsPoint,
    L: AsLineSegment,
{
    workplane: Entity<Workplane>,
    point_a: Entity<PA>,
    point_b: Entity<PB>,
    line: Entity<L>,
}

impl<PA, PB, L> SymmetricLine<PA, PB, L>
where
    PA: AsPoint,
    PB: AsPoint,
    L: AsLineSegment,
{
    pub fn new(
        workplane: Entity<Workplane>,
        point_a: Entity<PA>,
        point_b: Entity<PB>,
        line: Entity<L>,
    ) -> Self {
        Self {
            workplane,
            point_a,
            point_b,
            line,
        }
    }
}

impl<PA, PB, L> AsConstraintData for SymmetricLine<PA, PB, L>
where
    PA: AsPoint,
    PB: AsPoint,
    L: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_SYMMETRIC_LINE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

impl<PA, PB, L> TypeInfo for SymmetricLine<PA, PB, L>
where
    PA: AsPoint,
    PB: AsPoint,
    L: AsLineSegment,
{
    fn type_of() -> String {
        format!(
            "SymmetricLine < {}, {}, {} >",
            PA::type_of(),
            PB::type_of(),
            L::type_of()
        )
    }
}

impl<PA, PB, L> From<Slvs_Constraint> for SymmetricLine<PA, PB, L>
where
    PA: AsPoint,
    PB: AsPoint,
    L: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            workplane: Entity::new(value.wrkpl),
            point_a: Entity::new(value.ptA),
            point_b: Entity::new(value.ptB),
            line: Entity::new(value.entityA),
        }
    }
}
