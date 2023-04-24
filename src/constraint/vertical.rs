use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, SLVS_C_VERTICAL},
    element::AsHandle,
    entity::{AsEntityData, AsLineSegment, AsPoint, Entity},
};

////////////////////////////////////////////////////////////////////////////////
// From two points
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VerticalPoints<PA, PB>
where
    PA: AsPoint + AsEntityData,
    PB: AsPoint + AsEntityData,
{
    point_a: Entity<PA>,
    point_b: Entity<PB>,
}

impl<PA, PB> VerticalPoints<PA, PB>
where
    PA: AsPoint + AsEntityData,
    PB: AsPoint + AsEntityData,
{
    pub fn new(point_a: Entity<PA>, point_b: Entity<PB>) -> Self {
        Self { point_a, point_b }
    }
}

impl<PA, PB> AsConstraintData for VerticalPoints<PA, PB>
where
    PA: AsPoint + AsEntityData,
    PB: AsPoint + AsEntityData,
{
    fn type_(&self) -> i32 {
        SLVS_C_VERTICAL as _
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
    L: AsLineSegment + AsEntityData,
{
    line: Entity<L>,
}

impl<L> VerticalLine<L>
where
    L: AsLineSegment + AsEntityData,
{
    pub fn new(line: Entity<L>) -> Self {
        Self { line }
    }
}
impl<L> AsConstraintData for VerticalLine<L>
where
    L: AsLineSegment + AsEntityData,
{
    fn type_(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.as_handle()])
    }
}
