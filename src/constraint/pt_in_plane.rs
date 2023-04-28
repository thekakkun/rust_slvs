use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_PT_IN_PLANE},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct PtInPlane<P: AsPoint> {
    point: Entity<P>,
    plane: Entity<Workplane>,
}

impl<P: AsPoint> PtInPlane<P> {
    pub fn new(point: Entity<P>, plane: Entity<Workplane>) -> Self {
        Self { point, plane }
    }
}

impl<P: AsPoint> AsConstraintData for PtInPlane<P> {
    fn type_(&self) -> i32 {
        SLVS_C_PT_IN_PLANE as _
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
}

impl<P: AsPoint> TypeInfo for PtInPlane<P> {
    fn type_of() -> String {
        format!("PtInPlane< {} ", P::type_of())
    }
}

impl<P: AsPoint> From<Slvs_Constraint> for PtInPlane<P> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            point: Entity::new(value.ptA),
            plane: Entity::new(value.entityA),
        }
    }
}
