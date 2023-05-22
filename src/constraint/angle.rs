use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ANGLE},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Angle {
    pub group: Group,
    pub line_a: LineSegmentHandle,
    pub line_b: LineSegmentHandle,
    pub angle: f64,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl Angle {
    pub fn new(
        group: Group,
        line_a: LineSegmentHandle,
        line_b: LineSegmentHandle,
        angle: f64,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            line_b,
            angle,
            workplane,
        }
    }
}

impl AsConstraintData for Angle {
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
            angle: slvs_constraint.valA,
            workplane: match slvs_constraint.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_ANGLE as _
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

    fn val(&self) -> Option<f64> {
        Some(self.angle)
    }
}
