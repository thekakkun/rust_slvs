use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_LINE_ARC_LEN},
    element::AsHandle,
    entity::{ArcOfCircle, EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqualLineArcLen {
    pub group: Group,
    pub line: LineSegmentHandle,
    pub arc: EntityHandle<ArcOfCircle>,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl EqualLineArcLen {
    pub fn new(
        group: Group,
        line: LineSegmentHandle,
        arc: EntityHandle<ArcOfCircle>,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line,
            arc,
            workplane,
        }
    }
}

impl AsConstraintData for EqualLineArcLen {
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_LINE_ARC_LEN as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle(), self.arc.handle()])
    }
}

// impl<L: AsLineSegment> From<Slvs_Constraint> for EqualLineArcLen<L> {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             line: EntityHandle::new(value.entityA),
//             arc: EntityHandle::new(value.entityB),
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
//
