use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_PROJ_PT_DISTANCE},
    element::{AsHandle, TypeInfo},
    entity::{As2dProjectionTarget, AsPoint, Entity},
};

#[derive(Clone, Copy, Debug)]
pub struct ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    point_a: Entity<PA>,
    point_b: Entity<PB>,
    on_line: Entity<PT>,
    distance: f64,
}

impl<PA, PB, PT> ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    pub fn new(
        point_a: Entity<PA>,
        point_b: Entity<PB>,
        on_line: Entity<PT>,
        distance: f64,
    ) -> Self {
        Self {
            point_a,
            point_b,
            on_line,
            distance,
        }
    }
}

impl<PA, PB, PT> AsConstraintData for ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    fn type_(&self) -> i32 {
        SLVS_C_PROJ_PT_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.on_line.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

impl<PA, PB, PT> TypeInfo for ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    fn type_of() -> String {
        format!(
            "ProjPtDistance < {}, {}, {} >",
            PA::type_of(),
            PB::type_of(),
            PT::type_of()
        )
    }
}

impl<PA, PB, PT> From<Slvs_Constraint> for ProjPtDistance<PA, PB, PT>
where
    PA: AsPoint,
    PB: AsPoint,
    PT: As2dProjectionTarget,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            point_a: Entity::new(value.ptA),
            point_b: Entity::new(value.ptB),
            on_line: Entity::new(value.entityA),
            distance: value.valA,
        }
    }
}
