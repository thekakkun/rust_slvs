use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_RADIUS},
    element::{AsHandle, TypeInfo},
    entity::{AsArc, Entity},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    pub group: Group,
    pub arc_a: Entity<AA>,
    pub arc_b: Entity<AB>,
}

impl<AA, AB> EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    pub fn new(group: Group, arc_a: Entity<AA>, arc_b: Entity<AB>) -> Self {
        Self {
            group,
            arc_a,
            arc_b,
        }
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

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
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
        format!("EqualRadius < {}, {} >", AA::type_of(), AB::type_of())
    }
}

impl<AA, AB> From<Slvs_Constraint> for EqualRadius<AA, AB>
where
    AA: AsArc,
    AB: AsArc,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            arc_a: Entity::new(value.entityA),
            arc_b: Entity::new(value.entityB),
        }
    }
}
