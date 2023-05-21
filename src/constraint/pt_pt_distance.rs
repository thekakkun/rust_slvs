use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_PT_DISTANCE},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PtPtDistance {
    pub group: Group,
    pub point_a: PointHandle,
    pub point_b: PointHandle,
    pub distance: f64,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl PtPtDistance {
    pub fn new(
        group: Group,
        point_a: PointHandle,
        point_b: PointHandle,
        distance: f64,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            distance,
            workplane,
        }
    }
}

impl AsConstraintData for PtPtDistance {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_PT_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

// impl<PA, PB> From<Slvs_Constraint> for PtPtDistance<PA, PB>
// where
//     PA: AsPoint,
//     PB: AsPoint,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point_a: EntityHandle::new(value.ptA),
//             point_b: EntityHandle::new(value.ptB),
//             distance: value.valA,
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
