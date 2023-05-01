use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_LINE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct PtOnLine<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub group: Group,
    pub point: Entity<P>,
    pub line: Entity<L>,
    pub workplane: Option<Entity<Workplane>>,
}

impl<P, L> PtOnLine<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub fn new(
        group: Group,
        point: Entity<P>,
        line: Entity<L>,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            group,
            point,
            line,
            workplane,
        }
    }
}

impl<P, L> AsConstraintData for PtOnLine<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_PT_ON_LINE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

impl<P, L> TypeInfo for PtOnLine<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn type_of() -> String {
        format!("PtOnLine < {}, {} >", P::type_of(), L::type_of())
    }
}

impl<P, L> From<Slvs_Constraint> for PtOnLine<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point: Entity::new(value.ptA),
            line: Entity::new(value.entityA),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
