use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CURVE_CURVE_TANGENT},
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{AsCurve, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    fn new(
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

impl<CA, CB> AsGroup for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<CA, CB> AsSlvsType for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn slvs_type(&self) -> i32 {
        SLVS_C_CURVE_CURVE_TANGENT as _
    }
}

impl<CA, CB> AsConstraintData for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let curve_a = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
    //     let curve_b = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         workplane: EntityHandle::new(slvs_constraint.wrkpl),
    //         curve_a,
    //         curve_b,
    //         to_curve_a_start: slvs_constraint.other != 0,
    //         to_curve_b_start: slvs_constraint.other2 != 0,
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.curve_a.handle(), self.curve_b.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_curve_a_start, self.to_curve_b_start]
    }
}
