use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CUBIC_LINE_TANGENT},
    element::AsHandle,
    entity::{CubicHandle, EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CubicLineTangent {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub cubic: CubicHandle,
    pub line: LineSegmentHandle,
    pub to_start: bool,
}

impl CubicLineTangent {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        cubic: CubicHandle,
        line: LineSegmentHandle,
        to_start: bool,
    ) -> Self {
        Self {
            group,
            workplane,
            cubic,
            line,
            to_start,
        }
    }
}

impl AsConstraintData for CubicLineTangent {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let cubic = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
        let line = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            workplane: EntityHandle::new(slvs_constraint.wrkpl),
            cubic,
            line,
            to_start: slvs_constraint.other != 0,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_CUBIC_LINE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.cubic.handle(), self.line.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_start, false]
    }
}
