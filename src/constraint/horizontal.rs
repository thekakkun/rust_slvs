use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_HORIZONTAL},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, EntityHandle, Workplane},
    group::Group,
};

////////////////////////////////////////////////////////////////////////////////
// From two points
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub point_a: EntityHandle<PA>,
    pub point_b: EntityHandle<PB>,
}

impl<PA, PB> PointsHorizontal<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<PA>,
        point_b: EntityHandle<PB>,
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
            workplane: EntityHandle::new(value.wrkpl),
            point_a: EntityHandle::new(value.ptA),
            point_b: EntityHandle::new(value.ptB),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// From line segment
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LineHorizontal<L: AsLineSegment> {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub line: EntityHandle<L>,
}

impl<L: AsLineSegment> LineHorizontal<L> {
    pub fn new(group: Group, workplane: EntityHandle<Workplane>, line: EntityHandle<L>) -> Self {
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
            workplane: EntityHandle::new(value.wrkpl),
            line: EntityHandle::new(value.entityA),
        }
    }
}
