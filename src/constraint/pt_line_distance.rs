use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_LINE_DISTANCE},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtLineDistance {
    pub group: Group,
    pub point: PointHandle,
    pub line: LineSegmentHandle,
    pub distance: f64,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl PtLineDistance {
    pub fn new(
        group: Group,
        point: PointHandle,
        line: LineSegmentHandle,
        distance: f64,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            point,
            line,
            distance,
            workplane,
        }
    }
}

impl AsConstraintData for PtLineDistance {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
        let line = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            point,
            line,
            distance: slvs_constraint.valA,
            workplane: match slvs_constraint.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_LINE_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}
