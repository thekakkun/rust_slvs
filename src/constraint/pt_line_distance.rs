use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_LINE_DISTANCE},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtLineDistance {
    pub group: Group,
    pub point: PointHandle,
    pub line: LineSegmentHandle,
    pub distance: f64,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl PtLineDistance {
    pub fn new(
        group: Group,
        point: PointHandle,
        line: LineSegmentHandle,
        distance: f64,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            point,
            line,
            distance,
            workplane,
        }
    }
}

impl AsConstraintData for PtLineDistance {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_LINE_DISTANCE as _
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
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

// impl<P, L> From<Slvs_Constraint> for PtLineDistance<P, L>
// where
//     P: AsPoint,
//     L: AsLineSegment,
// {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point: EntityHandle::new(value.ptA),
//             line: EntityHandle::new(value.entityA),
//             distance: value.valA,
//             workplane: match value.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//         }
//     }
// }
