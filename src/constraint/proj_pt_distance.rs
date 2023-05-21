use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PROJ_PT_DISTANCE},
    element::AsHandle,
    entity::{PointHandle, ProjectionTargetHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ProjPtDistance {
    pub group: Group,
    pub point_a: PointHandle,
    pub point_b: PointHandle,
    pub on_line: ProjectionTargetHandle,
    pub distance: f64,
}

impl ProjPtDistance {
    pub fn new(
        group: Group,
        point_a: PointHandle,
        point_b: PointHandle,
        on_line: ProjectionTargetHandle,
        distance: f64,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            on_line,
            distance,
        }
    }
}

impl AsConstraintData for ProjPtDistance {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PROJ_PT_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.on_line.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

// impl<PA, PB, PT> From<Slvs_Constraint> for ProjPtDistance<PA, PB, PT>
// where
//     PA: AsPoint,
//     PB: AsPoint,
//     PT: As2dProjectionTarget,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point_a: EntityHandle::new(value.ptA),
//             point_b: EntityHandle::new(value.ptB),
//             on_line: EntityHandle::new(value.entityA),
//             distance: value.valA,
//         }
//     }
// }
