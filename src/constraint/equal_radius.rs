use crate::{
    bindings::{Slvs_hEntity, SLVS_C_EQUAL_RADIUS},
    element::AsHandle,
    entity::{AsArc, Entity},
};

use super::AsConstraintData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    arc_a: Entity<AA>,
    arc_b: Entity<AB>,
}

impl<AA, AB> EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    pub fn new(arc_a: Entity<AA>, arc_b: Entity<AB>) -> Self {
        Self { arc_a, arc_b }
    }
}

impl<AA, AB> AsConstraintData for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn type_(&self) -> i32 {
        SLVS_C_EQUAL_RADIUS as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.as_handle(), self.arc_b.as_handle()])
    }
}
