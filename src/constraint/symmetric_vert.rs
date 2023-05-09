use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC_VERT},
    element::AsHandle,
    entity::{AsPoint, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SymmetricVert<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub point_a: EntityHandle<PA>,
    pub point_b: EntityHandle<PB>,
}

impl<PA, PB> SymmetricVert<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<PA>,
        point_b: EntityHandle<PB>,
    ) -> Self {
        Self {
            group,
            workplane,
            point_a,
            point_b,
        }
    }
}

impl<PA, PB> AsConstraintData for SymmetricVert<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_SYMMETRIC_VERT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<PA, PB> From<Slvs_Constraint> for SymmetricVert<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            workplane: EntityHandle::new(value.wrkpl),
            point_a: EntityHandle::new(value.ptA),
            point_b: EntityHandle::new(value.ptB),
        }
    }
}
