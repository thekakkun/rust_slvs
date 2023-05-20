use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_PROJ_PT_DISTANCE},
    element::AsHandle,
    entity::{As2dProjectionTarget, AsPoint, EntityHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    pub group: Group,
    pub point_a: EntityHandle<PA>,
    pub point_b: EntityHandle<PB>,
    pub on_line: EntityHandle<PT>,
    pub distance: f64,
}

impl<PA, PB, PT> ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    pub fn new(
        group: Group,
        point_a: EntityHandle<PA>,
        point_b: EntityHandle<PB>,
        on_line: EntityHandle<PT>,
        distance: f64,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            on_line,
            distance,
        }
    }
}

impl<PA, PB, PT> AsConstraintData for ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    fn slvs_type(&self) -> i32 {
        SLVS_C_PROJ_PT_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.on_line.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

impl<PA, PB, PT> From<Slvs_Constraint> for ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point_a: EntityHandle::new(value.ptA),
            point_b: EntityHandle::new(value.ptB),
            on_line: EntityHandle::new(value.entityA),
            distance: value.valA,
        }
    }
}
