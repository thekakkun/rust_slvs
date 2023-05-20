use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_CURVE_CURVE_TANGENT},
    element::AsHandle,
    entity::{AsCurve, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub curve_a: EntityHandle<CA>,
    pub curve_b: EntityHandle<CB>,
    pub to_curve_a_start: bool,
    pub to_curve_b_start: bool,
}

impl<CA, CB> CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        curve_a: EntityHandle<CA>,
        curve_b: EntityHandle<CB>,
        to_curve_a_start: bool,
        to_curve_b_start: bool,
    ) -> Self {
        Self {
            group,
            workplane,
            curve_a,
            curve_b,
            to_curve_a_start,
            to_curve_b_start,
        }
    }
}

impl<CA, CB> AsConstraintData for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn slvs_type(&self) -> i32 {
        SLVS_C_CURVE_CURVE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.curve_a.handle(), self.curve_b.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_curve_a_start, self.to_curve_b_start]
    }
}

impl<CA, CB> From<Slvs_Constraint> for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            workplane: EntityHandle::new(value.wrkpl),
            curve_a: EntityHandle::new(value.entityA),
            curve_b: EntityHandle::new(value.entityB),
            to_curve_a_start: value.other != 0,
            to_curve_b_start: value.other2 != 0,
        }
    }
}
