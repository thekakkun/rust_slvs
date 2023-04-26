use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_HORIZONTAL},
    element::AsHandle,
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
};

////////////////////////////////////////////////////////////////////////////////
// From two points
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    workplane: Entity<Workplane>,
    point_a: Entity<PA>,
    point_b: Entity<PB>,
}

impl<PA, PB> PointsHorizontal<PA, PB>
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

impl<PA, PB> AsConstraintData for PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_HORIZONTAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.as_handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
    }
}

impl<PA, PB> From<Slvs_Constraint> for PointsHorizontal<PA, PB>
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
pub struct LineHorizontal<L: AsLineSegment> {
    workplane: Entity<Workplane>,
    line: Entity<L>,
}

impl<L: AsLineSegment> LineHorizontal<L> {
    pub fn new(workplane: Entity<Workplane>, line: Entity<L>) -> Self {
        Self { workplane, line }
    }
}

impl<L: AsLineSegment> AsConstraintData for LineHorizontal<L> {
    fn type_(&self) -> i32 {
        SLVS_C_HORIZONTAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.as_handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.as_handle()])
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for LineHorizontal<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            workplane: Entity::new(value.wrkpl),
            line: Entity::new(value.entityA),
        }
    }
}
