use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_CURVE_CURVE_TANGENT},
    element::{AsHandle, TypeInfo},
    entity::{AsCurve, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    workplane: Entity<Workplane>,
    curve_a: Entity<CA>,
    curve_b: Entity<CB>,
    to_curve_a_beginning: bool,
    to_curve_b_beginning: bool,
}

impl<CA, CB> CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    pub fn new(
        workplane: Entity<Workplane>,
        curve_a: Entity<CA>,
        curve_b: Entity<CB>,
        to_curve_a_beginning: bool,
        to_curve_b_beginning: bool,
    ) -> Self {
        Self {
            workplane,
            curve_a,
            curve_b,
            to_curve_a_beginning,
            to_curve_b_beginning,
        }
    }
}

impl<CA, CB> AsConstraintData for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn type_(&self) -> i32 {
        SLVS_C_CURVE_CURVE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.curve_a.handle(), self.curve_b.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_curve_a_beginning, self.to_curve_b_beginning]
    }
}

impl<CA, CB> TypeInfo for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn type_of() -> String {
        format!("CurveCurveTangent < {}, {} >", CA::type_of(), CB::type_of())
    }
}

impl<CA, CB> From<Slvs_Constraint> for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            workplane: Entity::new(value.wrkpl),
            curve_a: Entity::new(value.entityA),
            curve_b: Entity::new(value.entityB),
            to_curve_a_beginning: value.other != 0,
            to_curve_b_beginning: value.other2 != 0,
        }
    }
}
