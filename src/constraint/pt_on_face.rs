use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_FACE},
    element::AsHandle,
    entity::{AsPoint, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtOnFace<P: AsPoint> {
    pub group: Group,
    pub point: EntityHandle<P>,
    pub plane: EntityHandle<Workplane>,
}

impl<P: AsPoint> PtOnFace<P> {
    pub fn new(group: Group, point: EntityHandle<P>, plane: EntityHandle<Workplane>) -> Self {
        Self {
            group,
            point,
            plane,
        }
    }
}

impl<P: AsPoint> AsConstraintData for PtOnFace<P> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_FACE as _
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
}

impl<P: AsPoint> From<Slvs_Constraint> for PtOnFace<P> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point: EntityHandle::new(value.ptA),
            plane: EntityHandle::new(value.entityA),
        }
    }
}
