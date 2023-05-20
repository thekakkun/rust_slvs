use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_DIFFERENCE},
    element::AsHandle,
    entity::{ArcOfCircle, AsLineSegment, EntityHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcLineDifference<L: AsLineSegment> {
    pub group: Group,
    pub arc: EntityHandle<ArcOfCircle>,
    pub line: EntityHandle<L>,
    pub difference: f64,
}

impl<L: AsLineSegment> ArcLineDifference<L> {
    pub fn new(
        group: Group,
        arc: EntityHandle<ArcOfCircle>,
        line: EntityHandle<L>,
        difference: f64,
    ) -> Self {
        Self {
            group,
            arc,
            line,
            difference,
        }
    }
}

impl<L: AsLineSegment> AsConstraintData for ArcLineDifference<L> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_ARC_LINE_DIFFERENCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for ArcLineDifference<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            arc: EntityHandle::new(value.entityA),
            line: EntityHandle::new(value.entityB),
            difference: value.valA,
        }
    }
}
