use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_LINE_DISTANCE},
    element::AsHandle,
    entity::{AsEntityData, AsLineSegment, AsPoint, Entity},
};

use super::AsConstraintData;

pub struct PtLineDistance<P, L>
where
    P: AsPoint + AsEntityData,
    L: AsLineSegment + AsEntityData,
{
    point: Entity<P>,
    line: Entity<L>,
    distance: f64,
}

impl<P, L> PtLineDistance<P, L>
where
    P: AsPoint + AsEntityData,
    L: AsLineSegment + AsEntityData,
{
    pub fn new(point: Entity<P>, line: Entity<L>, distance: f64) -> Self {
        Self {
            point,
            line,
            distance,
        }
    }
}

impl<P, L> AsConstraintData for PtLineDistance<P, L>
where
    P: AsPoint + AsEntityData,
    L: AsLineSegment + AsEntityData,
{
    fn type_(&self) -> i32 {
        SLVS_C_PT_LINE_DISTANCE as _
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
