use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_RADIUS},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsArc, EntityHandle},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    pub group: Group,
    pub arc_a: EntityHandle<AA>,
    pub arc_b: EntityHandle<AB>,
}

impl<AA, AB> EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    pub fn new(group: Group, arc_a: EntityHandle<AA>, arc_b: EntityHandle<AB>) -> Self {
        Self {
            group,
            arc_a,
            arc_b,
        }
    }
}

impl<AA, AB> AsGroup for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<AA, AB> AsSlvsType for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_RADIUS as _
    }
}

impl<AA, AB> AsConstraintData for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc_a.handle(), self.arc_b.handle(), 0, 0])
    }
}

impl<AA, AB> FromSystem for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQUAL_RADIUS == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                arc_a: EntityHandle::new(slvs_constraint.entityA),
                arc_b: EntityHandle::new(slvs_constraint.entityB),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_EQUAL_RADIUS.")
        }
    }
}
