use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_ARC_LINE_DIFFERENCE},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, AsLineSegment, Entity},
};

#[derive(Clone, Copy, Debug)]
pub struct ArcLineDifference<L: AsLineSegment> {
    arc: Entity<ArcOfCircle>,
    line: Entity<L>,
    difference: f64,
}

impl<L: AsLineSegment> ArcLineDifference<L> {
    pub fn new(arc: Entity<ArcOfCircle>, line: Entity<L>, difference: f64) -> Self {
        Self {
            arc,
            line,
            difference,
        }
    }
}

impl<L: AsLineSegment> AsConstraintData for ArcLineDifference<L> {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_LINE_DIFFERENCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}

impl<L: AsLineSegment> TypeInfo for ArcLineDifference<L> {
    fn type_of() -> String {
        format!("ArcLineDifference<{}>", L::type_of())
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for ArcLineDifference<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            arc: Entity::new(value.entityA),
            line: Entity::new(value.entityB),
            difference: value.valA,
        }
    }
}
