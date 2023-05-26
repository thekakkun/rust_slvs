use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_RADIUS},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsRadiused, EntityHandle},
    group::Group,
    System,
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
    pub fn new(group: Group, radius_a: EntityHandle<RA>, radius_b: EntityHandle<RB>) -> Self {
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
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.radius_a.handle(), self.radius_b.handle()])
    }
}

impl<RA, RB> FromSystem for EqualRadius<RA, RB>
where
    RA: AsRadiused,
    RB: AsRadiused,
{
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQUAL_RADIUS == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                radius_a: EntityHandle::new(slvs_constraint.entityA),
                radius_b: EntityHandle::new(slvs_constraint.entityB),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_EQUAL_RADIUS.")
        }
    }
}
