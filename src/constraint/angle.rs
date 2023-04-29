use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_ANGLE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    line_a: Entity<LA>,
    line_b: Entity<LB>,
    angle: f64,
    workplane: Option<Entity<Workplane>>,
}

impl<LA, LB> Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    pub fn new(
        line_a: Entity<LA>,
        line_b: Entity<LB>,
        angle: f64,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            line_a,
            line_b,
            angle,
            workplane,
        }
    }
}

impl<LA, LB> AsConstraintData for Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_ANGLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line_a.handle(), self.line_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.angle)
    }
}

impl<LA, LB> TypeInfo for Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_of() -> String {
        format!("Angle < {}, {} >", LA::type_of(), LB::type_of())
    }
}

impl<LA, LB> From<Slvs_Constraint> for Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            line_a: Entity::new(value.entityA),
            line_b: Entity::new(value.entityB),
            angle: value.valA,
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
