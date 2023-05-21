use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_LEN_RATIO},
    element::AsHandle,
    entity::{ArcOfCircle, EntityHandle, LineSegmentHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcLineLenRatio {
    pub group: Group,
    pub arc: EntityHandle<ArcOfCircle>,
    pub line: LineSegmentHandle,
    pub ratio: f64,
}

impl ArcLineLenRatio {
    pub fn new(
        group: Group,
        arc: EntityHandle<ArcOfCircle>,
        line: LineSegmentHandle,
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

impl AsConstraintData for ArcLineLenRatio {
    fn slvs_type(&self) -> i32 {
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

// impl<L: AsLineSegment> From<Slvs_Constraint> for ArcLineLenRatio<L> {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             arc: EntityHandle::new(value.entityA),
//             line: EntityHandle::new(value.entityB),
//             ratio: value.valA,
//         }
//     }
// }
