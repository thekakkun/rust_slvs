use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SAME_ORIENTATION},
    element::AsHandle,
    entity::{EntityHandle, Normal},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SameOrientation {
    pub group: Group,
    pub normal_a: EntityHandle<Normal>,
    pub normal_b: EntityHandle<Normal>,
}

impl SameOrientation {
    pub fn new(
        group: Group,
        normal_a: EntityHandle<Normal>,
        normal_b: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            normal_a,
            normal_b,
        }
    }
}

impl AsConstraintData for SameOrientation {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            normal_a: EntityHandle::new(slvs_constraint.entityA),
            normal_b: EntityHandle::new(slvs_constraint.entityB),
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_SAME_ORIENTATION as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.normal_a.handle(), self.normal_b.handle()])
    }
}
