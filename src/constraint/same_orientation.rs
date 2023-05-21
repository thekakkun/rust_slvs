use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SAME_ORIENTATION},
    element::AsHandle,
    entity::{EntityHandle, Normal},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SameOrientation {
    pub group: Group,
    pub normal_a: EntityHandle<Normal>,
    pub normal_b: EntityHandle<Normal>,
}

impl SameOrientation {
    pub fn new(
        group: Group,
        normal_a: EntityHandle<Normal>,
        normal_b: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            normal_a,
            normal_b,
        }
    }
}

impl AsConstraintData for SameOrientation {
    fn slvs_type(&self) -> i32 {
        SLVS_C_SAME_ORIENTATION as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.normal_a.handle(), self.normal_b.handle()])
    }
}

// impl From<Slvs_Constraint> for SameOrientation {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             normal_a: EntityHandle::new(value.entityA),
//             normal_b: EntityHandle::new(value.entityB),
//         }
//     }
// }
