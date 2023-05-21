use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_DIAMETER},
    element::AsHandle,
    entity::ArcHandle,
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Diameter {
    pub group: Group,
    pub arc: ArcHandle,
    pub diameter: f64,
}

impl Diameter {
    pub fn new(group: Group, arc: ArcHandle, diameter: f64) -> Self {
        Self {
            group,
            arc,
            diameter,
        }
    }
}

impl AsConstraintData for Diameter {
    fn slvs_type(&self) -> i32 {
        SLVS_C_DIAMETER as _
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

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}

// impl From<Slvs_Constraint> for Diameter<A> {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             arc: EntityHandle::new(value.entityA),
//             diameter: value.valA,
//         }
//     }
// }
