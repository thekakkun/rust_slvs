use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_VERTICAL},
    element::AsElementIdentifier,
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
};

////////////////////////////////////////////////////////////////////////////////
// From two points
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointsVertical<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    workplane: Entity<Workplane>,
    point_a: Entity<PA>,
    point_b: Entity<PB>,
}

impl<PA, PB> PointsVertical<PA, PB>
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

impl<PA, PB> AsConstraintData for PointsVertical<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<PA, PB> From<Slvs_Constraint> for PointsVertical<PA, PB>
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

////////////////////////////////////////////////////////////////////////////////
// From line segment
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LineVertical<L: AsLineSegment> {
    workplane: Entity<Workplane>,
    line: Entity<L>,
}

impl<L: AsLineSegment> LineVertical<L> {
    pub fn new(workplane: Entity<Workplane>, line: Entity<L>) -> Self {
        Self { workplane, line }
    }
}

impl<L: AsLineSegment> AsConstraintData for LineVertical<L> {
    fn type_(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for LineVertical<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            workplane: Entity::new(value.wrkpl),
            line: Entity::new(value.entityA),
        }
    }
}
