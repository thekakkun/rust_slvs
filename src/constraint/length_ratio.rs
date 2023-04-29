use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_LENGTH_RATIO},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct LengthRatio<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    line_a: Entity<LA>,
    line_b: Entity<LB>,
    ratio: f64,
    workplane: Option<Entity<Workplane>>,
}

impl<LA, LB> LengthRatio<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    pub fn new(
        line_a: Entity<LA>,
        line_b: Entity<LB>,
        ratio: f64,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            line_a,
            line_b,
            ratio,
            workplane,
        }
    }
}

impl<LA, LB> AsConstraintData for LengthRatio<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_LENGTH_RATIO as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line_a.handle(), self.line_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

impl<LA, LB> TypeInfo for LengthRatio<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_of() -> String {
        format!("LengthRatio < {}, {} >", LA::type_of(), LB::type_of())
    }
}

impl<LA, LB> From<Slvs_Constraint> for LengthRatio<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            line_a: Entity::new(value.entityA),
            line_b: Entity::new(value.entityB),
            ratio: value.valA,
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
