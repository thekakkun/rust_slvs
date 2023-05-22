use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_ARC_LEN_RATIO},
    element::AsHandle,
    entity::{ArcOfCircle, EntityHandle},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArcArcLenRatio {
    pub group: Group,
    pub arc_a: EntityHandle<ArcOfCircle>,
    pub arc_b: EntityHandle<ArcOfCircle>,
    pub ratio: f64,
}

impl ArcArcLenRatio {
    pub fn new(
        group: Group,
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
        ratio: f64,
    ) -> Self {
        Self {
            group,
            arc_a,
            arc_b,
            ratio,
        }
    }
}

impl AsConstraintData for ArcArcLenRatio {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            arc_a: EntityHandle::new(slvs_constraint.entityA),
            arc_b: EntityHandle::new(slvs_constraint.entityB),
            ratio: slvs_constraint.valA,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_ARC_ARC_LEN_RATIO as _
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

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}
