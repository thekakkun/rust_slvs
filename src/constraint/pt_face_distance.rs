use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_PT_FACE_DISTANCE},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct PtFaceDistance<P: AsPoint> {
    point: Entity<P>,
    plane: Entity<Workplane>,
    distance: f64,
}

impl<P: AsPoint> PtFaceDistance<P> {
    pub fn new(point: Entity<P>, plane: Entity<Workplane>, distance: f64) -> Self {
        Self {
            point,
            plane,
            distance,
        }
    }
}

impl<P: AsPoint> AsConstraintData for PtFaceDistance<P> {
    fn type_(&self) -> i32 {
        SLVS_C_PT_FACE_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

impl<P: AsPoint> TypeInfo for PtFaceDistance<P> {
    fn type_of() -> String {
        format!("PtFaceDistance< {} >", P::type_of())
    }
}

impl<P: AsPoint> From<Slvs_Constraint> for PtFaceDistance<P> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            point: Entity::new(value.ptA),
            plane: Entity::new(value.entityA),
            distance: value.valA,
        }
    }
}
