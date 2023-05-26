use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CURVE_CURVE_TANGENT},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsCurve, EntityHandle, Workplane},
    group::Group,
    System,
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

impl<CA, CB> FromSystem for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_CURVE_CURVE_TANGENT == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                workplane: EntityHandle::new(slvs_constraint.wrkpl),
                curve_a: EntityHandle::new(slvs_constraint.entityA),
                curve_b: EntityHandle::new(slvs_constraint.entityB),
                to_curve_a_start: slvs_constraint.other != 0,
                to_curve_b_start: slvs_constraint.other2 != 0,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_CURVE_CURVE_TANGENT.")
        }
    }
}
