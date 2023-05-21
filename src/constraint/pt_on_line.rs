use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_LINE},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtOnLine {
    pub group: Group,
    pub point: PointHandle,
    pub line: LineSegmentHandle,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl PtOnLine {
    pub fn new(
        group: Group,
        point: PointHandle,
        line: LineSegmentHandle,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            point,
            line,
            workplane,
        }
    }
}

impl AsConstraintData for PtOnLine {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_LINE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

// impl<P, L> From<Slvs_Constraint> for PtOnLine<P, L>
// where
//     P: AsPoint,
//     L: AsLineSegment,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point: EntityHandle::new(value.ptA),
//             line: EntityHandle::new(value.entityA),
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
