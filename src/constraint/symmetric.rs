use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Symmetric {
    pub group: Group,
    pub point_a: PointHandle,
    pub point_b: PointHandle,
    pub plane: EntityHandle<Workplane>,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl Symmetric {
    pub fn new(
        group: Group,
        point_a: PointHandle,
        point_b: PointHandle,
        plane: EntityHandle<Workplane>,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            plane,
            workplane,
        }
    }
}

impl AsConstraintData for Symmetric {
    fn slvs_type(&self) -> i32 {
        SLVS_C_SYMMETRIC as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }
}

// impl<PA, PB> From<Slvs_Constraint> for Symmetric<PA, PB>
// where
//     PA: AsPoint,
//     PB: AsPoint,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point_a: EntityHandle::new(value.ptA),
//             point_b: EntityHandle::new(value.ptB),
//             plane: EntityHandle::new(value.entityA),
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
