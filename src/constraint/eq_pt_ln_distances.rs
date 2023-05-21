use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_PT_LN_DISTANCES},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqPtLnDistances {
    pub group: Group,
    pub line_a: LineSegmentHandle,
    pub point_a: PointHandle,
    pub line_b: LineSegmentHandle,
    pub point_b: PointHandle,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl EqPtLnDistances {
    pub fn new(
        group: Group,
        line_a: LineSegmentHandle,
        point_a: PointHandle,
        line_b: LineSegmentHandle,
        point_b: PointHandle,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            point_a,
            line_b,
            point_b,
            workplane,
        }
    }
}

impl AsConstraintData for EqPtLnDistances {
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQ_PT_LN_DISTANCES as _
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
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

// impl<LA, PA, LB, PB> From<Slvs_Constraint> for EqPtLnDistances<LA, PA, LB, PB>
// where
//     LA: AsLineSegment,
//     PA: AsPoint,
//     LB: AsLineSegment,
//     PB: AsPoint,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             line_a: EntityHandle::new(value.entityA),
//             point_a: EntityHandle::new(value.ptA),
//             line_b: EntityHandle::new(value.entityB),
//             point_b: EntityHandle::new(value.ptB),
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
