use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_EQ_LEN_PT_LINE_D},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct EqLenPtLineD<LA, P, LB>
where
    LA: AsLineSegment,
    P: AsPoint,
    LB: AsLineSegment,
{
    pub group: Group,
    pub line_a: Entity<LA>,
    pub point: Entity<P>,
    pub line_b: Entity<LB>,
    pub workplane: Option<Entity<Workplane>>,
}

impl<LA, P, LB> EqLenPtLineD<LA, P, LB>
where
    LA: AsLineSegment,
    P: AsPoint,
    LB: AsLineSegment,
{
    pub fn new(
        group: Group,
        line_a: Entity<LA>,
        point: Entity<P>,
        line_b: Entity<LB>,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            point,
            line_b,
            workplane,
        }
    }
}

impl<LA, P, LB> AsConstraintData for EqLenPtLineD<LA, P, LB>
where
    LA: AsLineSegment,
    P: AsPoint,
    LB: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_EQ_LEN_PT_LINE_D as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> u32 {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line_a.handle(), self.line_b.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}

impl<LA, P, LB> TypeInfo for EqLenPtLineD<LA, P, LB>
where
    LA: AsLineSegment,
    P: AsPoint,
    LB: AsLineSegment,
{
    fn type_of() -> String {
        format!(
            "EqLenPtLineD < {}, {}, {} >",
            LA::type_of(),
            P::type_of(),
            LB::type_of()
        )
    }
}

impl<LA, P, LB> From<Slvs_Constraint> for EqLenPtLineD<LA, P, LB>
where
    LA: AsLineSegment,
    P: AsPoint,
    LB: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            line_a: Entity::new(value.entityA),
            point: Entity::new(value.ptA),
            line_b: Entity::new(value.entityB),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
