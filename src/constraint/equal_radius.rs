use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_RADIUS},
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{AsRadiused, EntityHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EqualRadius<RA, RB>
where
    RA: AsRadiused,
    RB: AsRadiused,
{
    pub group: Group,
    pub radius_a: EntityHandle<RA>,
    pub radius_b: EntityHandle<RB>,
}

impl<RA, RB> EqualRadius<RA, RB>
where
    RA: AsRadiused,
    RB: AsRadiused,
{
    fn new(group: Group, radius_a: EntityHandle<RA>, radius_b: EntityHandle<RB>) -> Self {
        Self {
            group,
            radius_a,
            radius_b,
        }
    }
}

impl<RA, RB> AsGroup for EqualRadius<RA, RB>
where
    RA: AsRadiused,
    RB: AsRadiused,
{
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<RA, RB> AsSlvsType for EqualRadius<RA, RB>
where
    RA: AsRadiused,
    RB: AsRadiused,
{
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_RADIUS as _
    }
}

impl<RA, RB> AsConstraintData for EqualRadius<RA, RB>
where
    RA: AsRadiused,
    RB: AsRadiused,
{
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let arc_a = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
    //     let arc_b = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         radius_a: arc_a,
    //         radius_b: arc_b,
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.radius_a.handle(), self.radius_b.handle()])
    }
}
