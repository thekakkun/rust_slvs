use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_LINE_ARC_LEN},
    element::AsHandle,
    entity::{ArcOfCircle, EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqualLineArcLen {
    pub group: Group,
    pub line: LineSegmentHandle,
    pub arc: EntityHandle<ArcOfCircle>,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl EqualLineArcLen {
    pub fn new(
        group: Group,
        line: LineSegmentHandle,
        arc: EntityHandle<ArcOfCircle>,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line,
            arc,
            workplane,
        }
    }
}

impl AsConstraintData for EqualLineArcLen {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let line = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            line,
            arc: EntityHandle::new(slvs_constraint.entityB),
            workplane: match slvs_constraint.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_LINE_ARC_LEN as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle(), self.arc.handle()])
    }
}
