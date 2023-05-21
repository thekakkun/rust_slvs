use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_CIRCLE},
    element::AsHandle,
    entity::{ArcHandle, PointHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtOnCircle {
    pub group: Group,
    pub point: PointHandle,
    pub arc: ArcHandle,
}

impl PtOnCircle {
    pub fn new(group: Group, point: PointHandle, arc: ArcHandle) -> Self {
        Self { group, point, arc }
    }
}

impl AsConstraintData for PtOnCircle {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}

// impl<P, A> From<Slvs_Constraint> for PtOnCircle<P, A>
// where
//     P: AsPoint,
//     A: AsArc,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point: EntityHandle::new(value.ptA),
//             arc: EntityHandle::new(value.entityA),
//         }
//     }
// }
