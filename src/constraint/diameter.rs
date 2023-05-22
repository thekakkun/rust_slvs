use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_DIAMETER},
    element::AsHandle,
    entity::ArcHandle,
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diameter {
    pub group: Group,
    pub arc: ArcHandle,
    pub diameter: f64,
}

impl Diameter {
    pub fn new(group: Group, arc: ArcHandle, diameter: f64) -> Self {
        Self {
            group,
            arc,
            diameter,
        }
    }
}

impl AsConstraintData for Diameter {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let arc = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            arc,
            diameter: slvs_constraint.valA,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_DIAMETER as _
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

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}
