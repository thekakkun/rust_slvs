use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_PT_DISTANCE},
    element::AsHandle,
    entity::{AsPoint, EntityHandle, Workplane},
    group::Group,
};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub group: Group,
    pub point_a: EntityHandle<PA>,
    pub point_b: EntityHandle<PB>,
    pub distance: f64,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl<PA, PB> PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(
        group: Group,
        point_a: EntityHandle<PA>,
        point_b: EntityHandle<PB>,
        distance: f64,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            distance,
            workplane,
        }
    }
}

impl<PA, PB> AsConstraintData for PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_PT_PT_DISTANCE as _
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
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<PA, PB> From<Slvs_Constraint> for PtPtDistance<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point_a: EntityHandle::new(value.ptA),
            point_b: EntityHandle::new(value.ptB),
            distance: value.valA,
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        }
    }
}
