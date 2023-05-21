use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_LENGTH_RATIO},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LengthRatio {
    pub group: Group,
    pub line_a: LineSegmentHandle,
    pub line_b: LineSegmentHandle,
    pub ratio: f64,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl LengthRatio {
    pub fn new(
        group: Group,
        line_a: LineSegmentHandle,
        line_b: LineSegmentHandle,
        ratio: f64,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            line_b,
            ratio,
            workplane,
        }
    }
}

impl AsConstraintData for LengthRatio {
    fn slvs_type(&self) -> i32 {
        SLVS_C_LENGTH_RATIO as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line_a.handle(), self.line_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

// impl<LA, LB> From<Slvs_Constraint> for LengthRatio<LA, LB>
// where
//     LA: AsLineSegment,
//     LB: AsLineSegment,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             line_a: EntityHandle::new(value.entityA),
//             line_b: EntityHandle::new(value.entityB),
//             ratio: value.valA,
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
