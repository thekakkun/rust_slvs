use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_ANGLE},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqualAngle {
    pub group: Group,
    pub line_a: LineSegmentHandle,
    pub line_b: LineSegmentHandle,
    pub line_c: LineSegmentHandle,
    pub line_d: LineSegmentHandle,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl EqualAngle {
    pub fn new(
        group: Group,
        line_a: LineSegmentHandle,
        line_b: LineSegmentHandle,
        line_c: LineSegmentHandle,
        line_d: LineSegmentHandle,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            line_b,
            line_c,
            line_d,
            workplane,
        }
    }
}

impl AsConstraintData for EqualAngle {
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_ANGLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.line_a.handle(),
            self.line_b.handle(),
            self.line_c.handle(),
            self.line_d.handle(),
        ])
    }
}

// impl<LA, LB, LC, LD> From<Slvs_Constraint> for EqualAngle<LA, LB, LC, LD>
// where
//     LA: AsLineSegment,
//     LB: AsLineSegment,
//     LC: AsLineSegment,
//     LD: AsLineSegment,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             line_a: EntityHandle::new(value.entityA),
//             line_b: EntityHandle::new(value.entityB),
//             line_c: EntityHandle::new(value.entityC),
//             line_d: EntityHandle::new(value.entityD),
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
