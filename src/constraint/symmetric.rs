use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
    System,
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
            plane: EntityHandle::new(slvs_constraint.entityA),
            workplane: match slvs_constraint.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        })
    }

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
