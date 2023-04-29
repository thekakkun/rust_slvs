use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_POINTS_COINCIDENT},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct PointsCoincident<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    point_a: Entity<PA>,
    point_b: Entity<PB>,
    workplane: Option<Entity<Workplane>>,
}

impl<PA, PB> PointsCoincident<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    pub fn new(
        point_a: Entity<PA>,
        point_b: Entity<PB>,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            point_a,
            point_b,
            workplane,
        }
    }
}

impl<PA, PB> AsConstraintData for PointsCoincident<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_(&self) -> i32 {
        SLVS_C_POINTS_COINCIDENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<PA, PB> TypeInfo for PointsCoincident<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn type_of() -> String {
        format!("PointsCoincident < {}, {} >", PA::type_of(), PB::type_of())
    }
}

impl<PA, PB> From<Slvs_Constraint> for PointsCoincident<PA, PB>
where
    PA: AsPoint,
    PB: AsPoint,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            point_a: Entity::new(value.ptA),
            point_b: Entity::new(value.ptB),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
