use crate::{
    bindings::{Slvs_hEntity, SLVS_C_DIAMETER},
    element::AsHandle,
    entity::{AsArc, Entity},
};

use super::AsConstraintData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Diameter<A>
where
    A: AsArc,
{
    arc: Entity<A>,
    diameter: f64,
}

impl<A> Diameter<A>
where
    A: AsArc,
{
    pub fn new(arc: Entity<A>, diameter: f64) -> Self {
        Self { arc, diameter }
    }
}

impl<A> AsConstraintData for Diameter<A>
where
    A: AsArc,
{
    fn type_(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.as_handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}
