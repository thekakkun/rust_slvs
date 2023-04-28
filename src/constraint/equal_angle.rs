use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_EQUAL_ANGLE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct EqualAngle<LA, LB, LC, LD>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
    LC: AsLineSegment,
    LD: AsLineSegment,
{
    line_a: Entity<LA>,
    line_b: Entity<LB>,
    line_c: Entity<LC>,
    line_d: Entity<LD>,
    workplane: Option<Entity<Workplane>>,
}

impl<LA, LB, LC, LD> EqualAngle<LA, LB, LC, LD>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
    LC: AsLineSegment,
    LD: AsLineSegment,
{
    pub fn new(
        line_a: Entity<LA>,
        line_b: Entity<LB>,
        line_c: Entity<LC>,
        line_d: Entity<LD>,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            line_a,
            line_b,
            line_c,
            line_d,
            workplane,
        }
    }
}

impl<LA, LB, LC, LD> AsConstraintData for EqualAngle<LA, LB, LC, LD>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
    LC: AsLineSegment,
    LD: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_EQUAL_ANGLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.line_a.handle(),
            self.line_b.handle(),
            self.line_c.handle(),
            self.line_d.handle(),
        ])
    }
}

impl<LA, LB, LC, LD> TypeInfo for EqualAngle<LA, LB, LC, LD>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
    LC: AsLineSegment,
    LD: AsLineSegment,
{
    fn type_of() -> String {
        format!(
            "EqualAngle< {}, {}, {}, {},>",
            LA::type_of(),
            LB::type_of(),
            LC::type_of(),
            LD::type_of()
        )
    }
}

impl<LA, LB, LC, LD> From<Slvs_Constraint> for EqualAngle<LA, LB, LC, LD>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
    LC: AsLineSegment,
    LD: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            line_a: Entity::new(value.entityA),
            line_b: Entity::new(value.entityB),
            line_c: Entity::new(value.entityC),
            line_d: Entity::new(value.entityD),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
