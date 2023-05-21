use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CUBIC_LINE_TANGENT},
    element::AsHandle,
    entity::{CubicHandle, EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CubicLineTangent {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub cubic: CubicHandle,
    pub line: LineSegmentHandle,
    pub to_start: bool,
}

impl CubicLineTangent {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        cubic: CubicHandle,
        line: LineSegmentHandle,
        to_start: bool,
    ) -> Self {
        Self {
            group,
            workplane,
            cubic,
            line,
            to_start,
        }
    }
}

impl AsConstraintData for CubicLineTangent {
    fn slvs_type(&self) -> i32 {
        SLVS_C_CUBIC_LINE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.cubic.handle(), self.line.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_start, false]
    }
}

// impl<C, L> From<Slvs_Constraint> for CubicLineTangent<C, L>
// where
//     C: AsCubic,
//     L: AsLineSegment,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             workplane: EntityHandle::new(value.wrkpl),
//             cubic: EntityHandle::new(value.entityA),
//             line: EntityHandle::new(value.entityB),
//             to_start: value.other != 0,
//         }
//     }
// }
