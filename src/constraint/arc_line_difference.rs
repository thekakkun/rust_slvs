use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_DIFFERENCE},
    element::AsHandle,
    entity::{ArcOfCircle, EntityHandle, LineSegmentHandle},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcLineDifference {
    pub group: Group,
    pub arc: EntityHandle<ArcOfCircle>,
    pub line: LineSegmentHandle,
    pub difference: f64,
}

impl ArcLineDifference {
    pub fn new(
        group: Group,
        arc: EntityHandle<ArcOfCircle>,
        line: LineSegmentHandle,
        difference: f64,
    ) -> Self {
        Self {
            group,
            arc,
            line,
            difference,
        }
    }
}

impl AsConstraintData for ArcLineDifference {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let line = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            arc: EntityHandle::new(slvs_constraint.entityA),
            line,
            difference: slvs_constraint.valA,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_ARC_LINE_DIFFERENCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}
