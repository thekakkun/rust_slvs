use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_PT_LINE_DISTANCE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub group: Group,
    pub point: Entity<P>,
    pub line: Entity<L>,
    pub distance: f64,
    pub workplane: Option<Entity<Workplane>>,
}

impl<P, L> PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub fn new(
        group: Group,
        point: Entity<P>,
        line: Entity<L>,
        distance: f64,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            group,
            point,
            line,
            distance,
            workplane,
        }
    }
}

impl<P, L> AsConstraintData for PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_PT_LINE_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> u32 {
        self.group.handle()
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

impl<P, L> TypeInfo for PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn type_of() -> String {
        format!("PtLineDistance < {}, {} >", P::type_of(), L::type_of())
    }
}

impl<P, L> From<Slvs_Constraint> for PtLineDistance<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point: Entity::new(value.ptA),
            line: Entity::new(value.entityA),
            distance: value.valA,
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
