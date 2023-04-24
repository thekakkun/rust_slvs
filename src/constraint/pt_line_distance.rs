use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_LINE_DISTANCE},
    element::AsHandle,
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    point: Entity<P>,
    line: Entity<L>,
    distance: f64,
    workplane: Option<Entity<Workplane>>,
}

impl<P, L> PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub fn new(
        point: Entity<P>,
        line: Entity<L>,
        distance: f64,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            point,
            line,
            distance,
            workplane,
        }
    }
}

impl<P, L> AsConstraintData for PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_PT_LINE_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.as_handle())
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.as_handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.as_handle()])
    }
}
