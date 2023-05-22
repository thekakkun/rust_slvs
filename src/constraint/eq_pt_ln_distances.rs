use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_PT_LN_DISTANCES},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqPtLnDistances {
    pub group: Group,
    pub line_a: LineSegmentHandle,
    pub point_a: PointHandle,
    pub line_b: LineSegmentHandle,
    pub point_b: PointHandle,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl EqPtLnDistances {
    pub fn new(
        group: Group,
        line_a: LineSegmentHandle,
        point_a: PointHandle,
        line_b: LineSegmentHandle,
        point_b: PointHandle,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            point_a,
            line_b,
            point_b,
            workplane,
        }
    }
}

impl AsConstraintData for EqPtLnDistances {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let line_a = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
        let point_a = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
        let line_b = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;
        let point_b = (*sys.slvs_entity(slvs_constraint.ptB)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            line_a,
            point_a,
            line_b,
            point_b,
            workplane: match slvs_constraint.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_EQ_PT_LN_DISTANCES as _
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

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}
