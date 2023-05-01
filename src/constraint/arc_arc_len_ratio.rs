use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_ARC_ARC_LEN_RATIO},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, Entity},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct ArcArcLenRatio {
    pub group: Group,
    pub arc_a: Entity<ArcOfCircle>,
    pub arc_b: Entity<ArcOfCircle>,
    pub ratio: f64,
}

impl ArcArcLenRatio {
    pub fn new(
        group: Group,
        arc_a: Entity<ArcOfCircle>,
        arc_b: Entity<ArcOfCircle>,
        ratio: f64,
    ) -> Self {
        Self {
            group,
            arc_a,
            arc_b,
            ratio,
        }
    }
}

impl AsConstraintData for ArcArcLenRatio {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_ARC_LEN_RATIO as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> u32 {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.handle(), self.arc_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

impl TypeInfo for ArcArcLenRatio {
    fn type_of() -> String {
        "ArcArcLenRatio".to_string()
    }
}

impl From<Slvs_Constraint> for ArcArcLenRatio {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            arc_a: Entity::new(value.entityA),
            arc_b: Entity::new(value.entityB),
            ratio: value.valA,
        }
    }
}
