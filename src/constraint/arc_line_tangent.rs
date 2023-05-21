use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_TANGENT},
    element::AsHandle,
    entity::{ArcOfCircle, EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcLineTangent {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub arc: EntityHandle<ArcOfCircle>,
    pub line: LineSegmentHandle,
    pub to_start: bool,
}

impl ArcLineTangent {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        arc: EntityHandle<ArcOfCircle>,
        line: LineSegmentHandle,
        to_start: bool,
    ) -> Self {
        Self {
            group,
            workplane,
            arc,
            line,
            to_start,
        }
    }
}

impl AsConstraintData for ArcLineTangent {
    fn slvs_type(&self) -> i32 {
        SLVS_C_ARC_LINE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_start, false]
    }
}

// impl<L: AsLineSegment> From<Slvs_Constraint> for ArcLineTangent<L> {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             workplane: EntityHandle::new(value.wrkpl),
//             arc: EntityHandle::new(value.entityA),
//             line: EntityHandle::new(value.entityB),
//             to_start: value.other != 0,
//         }
//     }
// }
