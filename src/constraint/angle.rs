use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_ANGLE},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    pub group: Group,
    pub line_a: EntityHandle<LA>,
    pub line_b: EntityHandle<LB>,
    pub angle: f64,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl<LA, LB> Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    pub fn new(
        group: Group,
        line_a: EntityHandle<LA>,
        line_b: EntityHandle<LB>,
        angle: f64,
        workplane: Option<EntityHandle<Workplane>>,
    ) -> Self {
        Self {
            group,
            line_a,
            line_b,
            angle,
            workplane,
        }
    }
}

impl<LA, LB> AsConstraintData for Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_ANGLE as _
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
        Some(self.angle)
    }
}

impl<LA, LB> TypeInfo for Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn type_of() -> String {
        format!("Angle < {}, {} >", LA::type_of(), LB::type_of())
    }
}

impl<LA, LB> From<Slvs_Constraint> for Angle<LA, LB>
where
    LA: AsLineSegment,
    LB: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            line_a: EntityHandle::new(value.entityA),
            line_b: EntityHandle::new(value.entityB),
            angle: value.valA,
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        }
    }
}
