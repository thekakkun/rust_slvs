use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_LEN_PT_LINE_D},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqLenPtLineD {
    pub group: Group,
    pub line_a: LineSegmentHandle,
    pub point: PointHandle,
    pub line_b: LineSegmentHandle,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl EqLenPtLineD {
    pub fn new(
        group: Group,
        line_a: LineSegmentHandle,
        point: PointHandle,
        line_b: LineSegmentHandle,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            point,
            line_b,
            workplane,
        }
    }
}

impl AsConstraintData for EqLenPtLineD {
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQ_LEN_PT_LINE_D as _
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

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}

// impl<LA, P, LB> From<Slvs_Constraint> for EqLenPtLineD<LA, P, LB>
// where
//     LA: AsLineSegment,
//     P: AsPoint,
//     LB: AsLineSegment,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             line_a: EntityHandle::new(value.entityA),
//             point: EntityHandle::new(value.ptA),
//             line_b: EntityHandle::new(value.entityB),
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
