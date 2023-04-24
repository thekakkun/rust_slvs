use crate::{
    bindings::{Slvs_hEntity, SLVS_C_DIAMETER},
    element::AsHandle,
    entity::{AsArc, Entity},
};

use super::AsConstraintData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Diameter<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    arc_a: Entity<AA>,
    arc_b: Entity<AB>,
}

impl<AA, AB> Diameter<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    pub fn new(arc_a: Entity<AA>, arc_b: Entity<AB>) -> Self {
        Self { arc_a, arc_b }
    }
}

impl<AA, AB> AsConstraintData for Diameter<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn type_(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.as_handle(), self.arc_b.as_handle()])
    }
}
