use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_RADIUS},
    element::AsHandle,
    entity::ArcHandle,
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqualRadius {
    pub group: Group,
    pub arc_a: ArcHandle,
    pub arc_b: ArcHandle,
}

impl EqualRadius {
    pub fn new(group: Group, arc_a: ArcHandle, arc_b: ArcHandle) -> Self {
        Self {
            group,
            arc_a,
            arc_b,
        }
    }
}

impl AsConstraintData for EqualRadius {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let arc_a = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
        let arc_b = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            arc_a,
            arc_b,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_RADIUS as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.handle(), self.arc_b.handle()])
    }
}
