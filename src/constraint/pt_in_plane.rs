use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_IN_PLANE},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PtInPlane {
    pub group: Group,
    pub point: PointHandle,
    pub plane: EntityHandle<Workplane>,
}

impl PtInPlane {
    pub fn new(group: Group, point: PointHandle, plane: EntityHandle<Workplane>) -> Self {
        Self {
            group,
            point,
            plane,
        }
    }
}

impl AsConstraintData for PtInPlane {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            point,
            plane: EntityHandle::new(slvs_constraint.entityA),
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_IN_PLANE as _
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
