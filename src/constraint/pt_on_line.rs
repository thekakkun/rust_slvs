use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_LINE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtOnLine<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub group: Group,
    pub point: EntityHandle<P>,
    pub line: EntityHandle<L>,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl<P, L> PtOnLine<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub fn new(
        group: Group,
        point: EntityHandle<P>,
        line: EntityHandle<L>,
        workplane: Option<EntityHandle<Workplane>>,
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
            point: EntityHandle::new(value.ptA),
            line: EntityHandle::new(value.entityA),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        }
    }
}
