use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, SLVS_C_VERTICAL},
    element::AsHandle,
    entity::{AsLineSegment, AsPoint, Entity},
};

////////////////////////////////////////////////////////////////////////////////
// From two points
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VerticalPoints<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    point_a: Entity<PA>,
    point_b: Entity<PB>,
}

impl<PA, PB> VerticalPoints<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(point_a: Entity<PA>, point_b: Entity<PB>) -> Self {
        Self { point_a, point_b }
    }
}

impl<PA, PB> AsConstraintData for VerticalPoints<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
    }
}

////////////////////////////////////////////////////////////////////////////////
// From line segment
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VerticalLine<L>
where
    L: AsLineSegment,
{
    line: Entity<L>,
}

impl<L> VerticalLine<L>
where
    L: AsLineSegment,
{
    pub fn new(line: Entity<L>) -> Self {
        Self { line }
    }
}
impl<L> AsConstraintData for VerticalLine<L>
where
    L: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.as_handle()])
    }
}
