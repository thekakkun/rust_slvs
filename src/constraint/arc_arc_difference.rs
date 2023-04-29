use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_ARC_ARC_DIFFERENCE},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, Entity},
};

#[derive(Clone, Copy, Debug)]
pub struct ArcArcDifference {
    arc_a: Entity<ArcOfCircle>,
    arc_b: Entity<ArcOfCircle>,
    difference: f64,
}

impl ArcArcDifference {
    pub fn new(arc_a: Entity<ArcOfCircle>, arc_b: Entity<ArcOfCircle>, difference: f64) -> Self {
        Self {
            arc_a,
            arc_b,
            difference,
        }
    }
}

impl AsConstraintData for ArcArcDifference {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_ARC_DIFFERENCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.handle(), self.arc_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}

impl TypeInfo for ArcArcDifference {
    fn type_of() -> String {
        "ArcArcDifference".to_string()
    }
}

impl From<Slvs_Constraint> for ArcArcDifference {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            arc_a: Entity::new(value.entityA),
            arc_b: Entity::new(value.entityB),
            difference: value.valA,
        }
    }
}
