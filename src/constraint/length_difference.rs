use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_LENGTH_DIFFERENCE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct LengthDifference<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    pub group: Group,
    pub line_a: Entity<LA>,
    pub line_b: Entity<LB>,
    pub difference: f64,
    pub workplane: Option<Entity<Workplane>>,
}

impl<LA, LB> LengthDifference<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    pub fn new(
        group: Group,
        line_a: Entity<LA>,
        line_b: Entity<LB>,
        difference: f64,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            line_b,
            difference,
            workplane,
        }
    }
}

impl<LA, LB> AsConstraintData for LengthDifference<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_LENGTH_DIFFERENCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line_a.handle(), self.line_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}

impl<LA, LB> TypeInfo for LengthDifference<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_of() -> String {
        format!("LengthDifference < {}, {} >", LA::type_of(), LB::type_of())
    }
}

impl<LA, LB> From<Slvs_Constraint> for LengthDifference<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            line_a: Entity::new(value.entityA),
            line_b: Entity::new(value.entityB),
            difference: value.valA,
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
