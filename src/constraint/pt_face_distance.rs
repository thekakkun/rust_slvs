use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_FACE_DISTANCE},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtFaceDistance {
    pub group: Group,
    pub point: PointHandle,
    pub plane: EntityHandle<Workplane>,
    pub distance: f64,
}

impl PtFaceDistance {
    pub fn new(
        group: Group,
        point: PointHandle,
        plane: EntityHandle<Workplane>,
        distance: f64,
    ) -> Self {
        Self {
            group,
            point,
            plane,
            distance,
        }
    }
}

impl AsConstraintData for PtFaceDistance {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_FACE_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

// impl<P: AsPoint> From<Slvs_Constraint> for PtFaceDistance<P> {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point: EntityHandle::new(value.ptA),
//             plane: EntityHandle::new(value.entityA),
//             distance: value.valA,
//         }
//     }
// }
