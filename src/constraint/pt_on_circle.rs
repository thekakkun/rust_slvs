use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_CIRCLE},
    element::AsHandle,
    entity::{ArcHandle, PointHandle},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtOnCircle {
    pub group: Group,
    pub point: PointHandle,
    pub arc: ArcHandle,
}

impl PtOnCircle {
    pub fn new(group: Group, point: PointHandle, arc: ArcHandle) -> Self {
        Self { group, point, arc }
    }
}

impl AsConstraintData for PtOnCircle {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
        let arc = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            point,
            arc,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_CIRCLE as _
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

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}
