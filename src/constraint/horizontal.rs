use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_HORIZONTAL},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
    group::Group,
};

////////////////////////////////////////////////////////////////////////////////
// From two points
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub group: Group,
    pub workplane: Entity<Workplane>,
    pub point_a: Entity<PA>,
    pub point_b: Entity<PB>,
}

impl<PA, PB> PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(
        group: Group,
        workplane: Entity<Workplane>,
        point_a: Entity<PA>,
        point_b: Entity<PB>,
    ) -> Self {
        Self {
            group,
            workplane,
            point_a,
            point_b,
        }
    }
}

impl<PA, PB> AsConstraintData for PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_HORIZONTAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<PA, PB> TypeInfo for PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_of() -> String {
        format!("Horizontal < {}, {} >", PA::type_of(), PB::type_of())
    }
}

impl<PA, PB> From<Slvs_Constraint> for PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            workplane: Entity::new(value.wrkpl),
            point_a: Entity::new(value.ptA),
            point_b: Entity::new(value.ptB),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// From line segment
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct LineHorizontal<L: AsLineSegment> {
    pub group: Group,
    pub workplane: Entity<Workplane>,
    pub line: Entity<L>,
}

impl<L: AsLineSegment> LineHorizontal<L> {
    pub fn new(group: Group, workplane: Entity<Workplane>, line: Entity<L>) -> Self {
        Self {
            group,
            workplane,
            line,
        }
    }
}

impl<L: AsLineSegment> AsConstraintData for LineHorizontal<L> {
    fn type_(&self) -> i32 {
        SLVS_C_HORIZONTAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}

impl<L: AsLineSegment> TypeInfo for LineHorizontal<L> {
    fn type_of() -> String {
        format!("Horizontal < {} >", L::type_of())
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for LineHorizontal<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            workplane: Entity::new(value.wrkpl),
            line: Entity::new(value.entityA),
        }
    }
}
