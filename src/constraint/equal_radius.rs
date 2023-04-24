use crate::{
    bindings::{Slvs_hEntity, SLVS_C_EQUAL_RADIUS},
    element::AsHandle,
    entity::{AsArc, AsEntityData, Entity},
};

use super::AsConstraintData;

pub struct EqualRadius<AA, AB>
where
    AA: AsArc + AsEntityData,
    AB: AsArc + AsEntityData,
{
    arc_a: Entity<AA>,
    arc_b: Entity<AB>,
}

impl<AA, AB> EqualRadius<AA, AB>
where
    AA: AsArc + AsEntityData,
    AB: AsArc + AsEntityData,
{
    pub fn new(arc_a: Entity<AA>, arc_b: Entity<AB>) -> Self {
        Self { arc_a, arc_b }
    }
}

impl<AA, AB> AsConstraintData for EqualRadius<AA, AB>
where
    AA: AsArc + AsEntityData,
    AB: AsArc + AsEntityData,
{
    fn type_(&self) -> i32 {
        SLVS_C_EQUAL_RADIUS as _
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.as_handle(), self.arc_b.as_handle()])
    }
}
