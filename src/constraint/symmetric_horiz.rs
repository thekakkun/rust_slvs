use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC_HORIZ},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymmetricHoriz {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub point_a: PointHandle,
    pub point_b: PointHandle,
}

impl SymmetricHoriz {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: PointHandle,
        point_b: PointHandle,
    ) -> Self {
        Self {
            group,
            workplane,
            point_a,
            point_b,
        }
    }
}

impl AsConstraintData for SymmetricHoriz {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let point_a = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
        let point_b = (*sys.slvs_entity(slvs_constraint.ptB)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            workplane: EntityHandle::new(slvs_constraint.wrkpl),
            point_a,
            point_b,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_SYMMETRIC_HORIZ as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}
