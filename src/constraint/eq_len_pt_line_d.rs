use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_LEN_PT_LINE_D},
    element::AsHandle,
    entity::{AsLineSegment, AsPoint, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EqLenPtLineD<LA, P, LB>
where
    LA: AsLineSegment,
    P: AsPoint,
    LB: AsLineSegment,
{
    pub group: Group,
    pub line_a: EntityHandle<LA>,
    pub point: EntityHandle<P>,
    pub line_b: EntityHandle<LB>,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl<LA, P, LB> EqLenPtLineD<LA, P, LB>
where
    LA: AsLineSegment,
    P: AsPoint,
    LB: AsLineSegment,
{
    pub fn new(
        group: Group,
        line_a: EntityHandle<LA>,
        point: EntityHandle<P>,
        line_b: EntityHandle<LB>,
        workplane: Option<EntityHandle<Workplane>>,
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
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQ_LEN_PT_LINE_D as _
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
        Some(vec![self.point.handle()])
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
            line_a: EntityHandle::new(value.entityA),
            point: EntityHandle::new(value.ptA),
            line_b: EntityHandle::new(value.entityB),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        }
    }
}
