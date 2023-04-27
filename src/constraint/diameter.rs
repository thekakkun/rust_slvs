use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_DIAMETER},
    element::AsElementIdentifier,
    entity::{AsArc, Entity},
};

use super::AsConstraintData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Diameter<A: AsArc> {
    arc: Entity<A>,
    diameter: f64,
}

impl<A: AsArc> Diameter<A> {
    pub fn new(arc: Entity<A>, diameter: f64) -> Self {
        Self { arc, diameter }
    }
}

impl<A: AsArc> AsConstraintData for Diameter<A> {
    fn type_(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}

impl<A: AsArc> From<Slvs_Constraint> for Diameter<A> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            arc: Entity::new(value.entityA),
            diameter: value.valA,
        }
    }
}
