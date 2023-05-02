use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_CIRCLE},
    element::{AsHandle, TypeInfo},
    entity::{AsArc, AsPoint, Entity},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtOnCircle<P, A>
where
    P: AsPoint,
    A: AsArc,
{
    pub group: Group,
    pub point: Entity<P>,
    pub arc: Entity<A>,
}

impl<P, A> PtOnCircle<P, A>
where
    P: AsPoint,
    A: AsArc,
{
    pub fn new(group: Group, point: Entity<P>, arc: Entity<A>) -> Self {
        Self { group, point, arc }
    }
}

impl<P, A> AsConstraintData for PtOnCircle<P, A>
where
    P: AsPoint,
    A: AsArc,
{
    fn type_(&self) -> i32 {
        SLVS_C_PT_ON_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}

impl<P, A> TypeInfo for PtOnCircle<P, A>
where
    P: AsPoint,
    A: AsArc,
{
    fn type_of() -> String {
        format!("PtOnCircle < {}, {} >", P::type_of(), A::type_of())
    }
}

impl<P, A> From<Slvs_Constraint> for PtOnCircle<P, A>
where
    P: AsPoint,
    A: AsArc,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point: Entity::new(value.ptA),
            arc: Entity::new(value.entityA),
        }
    }
}
