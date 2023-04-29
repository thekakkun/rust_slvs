use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_PT_ON_CIRCLE},
    element::{AsHandle, TypeInfo},
    entity::{AsArc, AsPoint, Entity},
};

#[derive(Clone, Copy, Debug)]
pub struct PtOnCircle<P, A>
where
    P: AsPoint,
    A: AsArc,
{
    point: Entity<P>,
    arc: Entity<A>,
}

impl<P, A> PtOnCircle<P, A>
where
    P: AsPoint,
    A: AsArc,
{
    pub fn new(point: Entity<P>, arc: Entity<A>) -> Self {
        Self { point, arc }
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
            point: Entity::new(value.ptA),
            arc: Entity::new(value.entityA),
        }
    }
}
