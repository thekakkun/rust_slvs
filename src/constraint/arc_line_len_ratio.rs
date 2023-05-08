use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_LEN_RATIO},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, AsLineSegment, EntityHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcLineLenRatio<L: AsLineSegment> {
    pub group: Group,
    pub arc: EntityHandle<ArcOfCircle>,
    pub line: EntityHandle<L>,
    pub ratio: f64,
}

impl<L: AsLineSegment> ArcLineLenRatio<L> {
    pub fn new(
        group: Group,
        arc: EntityHandle<ArcOfCircle>,
        line: EntityHandle<L>,
        ratio: f64,
    ) -> Self {
        Self {
            group,
            arc,
            line,
            ratio,
        }
    }
}

impl<L: AsLineSegment> AsConstraintData for ArcLineLenRatio<L> {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_LINE_LEN_RATIO as _
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
            group: Group(value.group),
            arc: EntityHandle::new(value.entityA),
            line: EntityHandle::new(value.entityB),
            ratio: value.valA,
        }
    }
}
