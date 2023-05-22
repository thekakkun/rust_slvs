use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_LENGTH_LINES},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EqualLengthLines {
    pub group: Group,
    pub line_a: LineSegmentHandle,
    pub line_b: LineSegmentHandle,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl EqualLengthLines {
    pub fn new(
        group: Group,
        line_a: LineSegmentHandle,
        line_b: LineSegmentHandle,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            line_b,
            workplane,
        }
    }
}

impl AsConstraintData for EqualLengthLines {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let line_a = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
        let line_b = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            line_a,
            line_b,
            workplane: match slvs_constraint.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_LENGTH_LINES as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line_a.handle(), self.line_b.handle()])
    }
}
