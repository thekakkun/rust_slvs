use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PROJ_PT_DISTANCE},
    element::AsHandle,
    entity::{PointHandle, ProjectionTargetHandle},
    group::Group,
    System,
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
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let point_a = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
        let point_b = (*sys.slvs_entity(slvs_constraint.ptB)?).try_into()?;
        let on_line = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            point_a,
            point_b,
            on_line,
            distance: slvs_constraint.valA,
        })
    }

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
