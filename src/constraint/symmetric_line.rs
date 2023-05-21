use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC_LINE},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SymmetricLine {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub point_a: PointHandle,
    pub point_b: PointHandle,
    pub line: LineSegmentHandle,
}

impl SymmetricLine {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: PointHandle,
        point_b: PointHandle,
        line: LineSegmentHandle,
    ) -> Self {
        Self {
            group,
            workplane,
            point_a,
            point_b,
            line,
        }
    }
}

impl AsConstraintData for SymmetricLine {
    fn slvs_type(&self) -> i32 {
        SLVS_C_SYMMETRIC_LINE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

// impl<PA, PB, L> From<Slvs_Constraint> for SymmetricLine<PA, PB, L>
// where
//     PA: AsPoint,
//     PB: AsPoint,
//     L: AsLineSegment,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             workplane: EntityHandle::new(value.wrkpl),
//             point_a: EntityHandle::new(value.ptA),
//             point_b: EntityHandle::new(value.ptB),
//             line: EntityHandle::new(value.entityA),
//         }
//     }
// }
