use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_PT_LN_DISTANCES},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct EqPtLnDistances<LA, PA, LB, PB>
where
    LA: AsLineSegment,
    PA: AsPoint,
    LB: AsLineSegment,
    PB: AsPoint,
{
    pub group: Group,
    pub line_a: Entity<LA>,
    pub point_a: Entity<PA>,
    pub line_b: Entity<LB>,
    pub point_b: Entity<PB>,
    pub workplane: Option<Entity<Workplane>>,
}

impl<LA, PA, LB, PB> EqPtLnDistances<LA, PA, LB, PB>
where
    LA: AsLineSegment,
    PA: AsPoint,
    LB: AsLineSegment,
    PB: AsPoint,
{
    pub fn new(
        group: Group,
        line_a: Entity<LA>,
        point_a: Entity<PA>,
        line_b: Entity<LB>,
        point_b: Entity<PB>,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            point_a,
            line_b,
            point_b,
            workplane,
        }
    }
}

impl<LA, PA, LB, PB> AsConstraintData for EqPtLnDistances<LA, PA, LB, PB>
where
    LA: AsLineSegment,
    PA: AsPoint,
    LB: AsLineSegment,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_EQ_PT_LN_DISTANCES as _
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

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<LA, PA, LB, PB> TypeInfo for EqPtLnDistances<LA, PA, LB, PB>
where
    LA: AsLineSegment,
    PA: AsPoint,
    LB: AsLineSegment,
    PB: AsPoint,
{
    fn type_of() -> String {
        format!(
            "EqLenPtLineD < {}, {}, {}, {} >",
            LA::type_of(),
            PA::type_of(),
            LB::type_of(),
            PB::type_of(),
        )
    }
}

impl<LA, PA, LB, PB> From<Slvs_Constraint> for EqPtLnDistances<LA, PA, LB, PB>
where
    LA: AsLineSegment,
    PA: AsPoint,
    LB: AsLineSegment,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            line_a: Entity::new(value.entityA),
            point_a: Entity::new(value.ptA),
            line_b: Entity::new(value.entityB),
            point_b: Entity::new(value.ptB),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
