use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_ARC_LINE_LEN_RATIO},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, AsLineSegment, Entity},
};

#[derive(Clone, Copy, Debug)]
pub struct ArcLineLenRatio<L: AsLineSegment> {
    arc: Entity<ArcOfCircle>,
    line: Entity<L>,
    ratio: f64,
}

impl<L: AsLineSegment> ArcLineLenRatio<L> {
    pub fn new(arc: Entity<ArcOfCircle>, line: Entity<L>, ratio: f64) -> Self {
        Self { arc, line, ratio }
    }
}

impl<L: AsLineSegment> AsConstraintData for ArcLineLenRatio<L> {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_LINE_LEN_RATIO as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

impl<L: AsLineSegment> TypeInfo for ArcLineLenRatio<L> {
    fn type_of() -> String {
        format!("ArcLineLenRatio<{}>", L::type_of())
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for ArcLineLenRatio<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            arc: Entity::new(value.entityA),
            line: Entity::new(value.entityB),
            ratio: value.valA,
        }
    }
}
