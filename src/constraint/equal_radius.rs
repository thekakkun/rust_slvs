use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_EQUAL_RADIUS},
    element::{AsHandle, TypeInfo},
    entity::{AsArc, Entity},
};

#[derive(Clone, Copy, Debug)]
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
        Some(vec![self.arc_a.handle(), self.arc_b.handle()])
    }
}

impl<AA, AB> TypeInfo for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn type_of() -> String {
        format!("EqualRadius< {}, {} >", AA::type_of(), AB::type_of())
    }
}

impl<AA, AB> From<Slvs_Constraint> for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            arc_a: Entity::new(value.entityA),
            arc_b: Entity::new(value.entityB),
        }
    }
}
