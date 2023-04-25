use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, SLVS_C_HORIZONTAL},
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

////////////////////////////////////////////////////////////////////////////////
// From line segment
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LineHorizontal<L>
where
    L: AsLineSegment,
{
    workplane: Entity<Workplane>,
    line: Entity<L>,
}

impl<L> LineHorizontal<L>
where
    L: AsLineSegment,
{
    pub fn new(workplane: Entity<Workplane>, line: Entity<L>) -> Self {
        Self { workplane, line }
    }
}

impl<L> AsConstraintData for LineHorizontal<L>
where
    L: AsLineSegment,
{
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
