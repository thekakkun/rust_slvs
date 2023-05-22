use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_PT_DISTANCE},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
    System,
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
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let point_a = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
        let point_b = (*sys.slvs_entity(slvs_constraint.ptB)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            point_a,
            point_b,
            distance: slvs_constraint.valA,
            workplane: match slvs_constraint.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        })
    }

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
