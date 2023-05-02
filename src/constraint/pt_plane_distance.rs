use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_PLANE_DISTANCE},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtPlaneDistance<P: AsPoint> {
    pub group: Group,
    pub point: Entity<P>,
    pub plane: Entity<Workplane>,
    pub distance: f64,
}

impl<P: AsPoint> PtPlaneDistance<P> {
    pub fn new(group: Group, point: Entity<P>, plane: Entity<Workplane>, distance: f64) -> Self {
        Self {
            group,
            point,
            plane,
            distance,
        }
    }
}

impl<P: AsPoint> AsConstraintData for PtPlaneDistance<P> {
    fn type_(&self) -> i32 {
        SLVS_C_PT_PLANE_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
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

impl<P: AsPoint> TypeInfo for PtPlaneDistance<P> {
    fn type_of() -> String {
        format!("PtPlaneDistance < {} >", P::type_of())
    }
}

impl<P: AsPoint> From<Slvs_Constraint> for PtPlaneDistance<P> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point: Entity::new(value.ptA),
            plane: Entity::new(value.entityA),
            distance: value.valA,
        }
    }
}
