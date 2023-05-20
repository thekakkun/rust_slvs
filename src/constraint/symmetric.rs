use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC},
    element::AsHandle,
    entity::{AsPoint, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Symmetric<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub group: Group,
    pub point_a: EntityHandle<PA>,
    pub point_b: EntityHandle<PB>,
    pub plane: EntityHandle<Workplane>,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl<PA, PB> Symmetric<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(
        group: Group,
        point_a: EntityHandle<PA>,
        point_b: EntityHandle<PB>,
        plane: EntityHandle<Workplane>,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            plane,
            workplane,
        }
    }
}

impl<PA, PB> AsConstraintData for Symmetric<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn slvs_type(&self) -> i32 {
        SLVS_C_SYMMETRIC as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }
}

impl<PA, PB> From<Slvs_Constraint> for Symmetric<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point_a: EntityHandle::new(value.ptA),
            point_b: EntityHandle::new(value.ptB),
            plane: EntityHandle::new(value.entityA),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        }
    }
}
