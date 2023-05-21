use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC_HORIZ},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SymmetricHoriz {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub point_a: PointHandle,
    pub point_b: PointHandle,
}

impl SymmetricHoriz {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: PointHandle,
        point_b: PointHandle,
    ) -> Self {
        Self {
            group,
            workplane,
            point_a,
            point_b,
        }
    }
}

impl AsConstraintData for SymmetricHoriz {
    fn slvs_type(&self) -> i32 {
        SLVS_C_SYMMETRIC_HORIZ as _
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
}

// impl<PA, PB> From<Slvs_Constraint> for SymmetricHoriz<PA, PB>
// where
//     PA: AsPoint,
//     PB: AsPoint,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             workplane: EntityHandle::new(value.wrkpl),
//             point_a: EntityHandle::new(value.ptA),
//             point_b: EntityHandle::new(value.ptB),
//         }
//     }
// }
