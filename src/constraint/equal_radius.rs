use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_RADIUS},
    element::AsHandle,
    entity::ArcHandle,
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqualRadius {
    pub group: Group,
    pub arc_a: ArcHandle,
    pub arc_b: ArcHandle,
}

impl EqualRadius {
    pub fn new(group: Group, arc_a: ArcHandle, arc_b: ArcHandle) -> Self {
        Self {
            group,
            arc_a,
            arc_b,
        }
    }
}

impl AsConstraintData for EqualRadius {
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_RADIUS as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.handle(), self.arc_b.handle()])
    }
}

// impl<AA, AB> From<Slvs_Constraint> for EqualRadius<AA, AB>
// where
//     AA: AsArc,
//     AB: AsArc,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             arc_a: EntityHandle::new(value.entityA),
//             arc_b: EntityHandle::new(value.entityB),
//         }
//     }
// }
