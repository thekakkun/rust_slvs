use super::{AsEntity, Distance, Entity, NormalIn2d, PointIn2d};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CIRCLE},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub normal: Entity<NormalIn2d>,
    pub center: Entity<PointIn2d>,
    pub radius: Entity<Distance>,
}

impl Circle {
    pub fn new(
        normal: Entity<NormalIn2d>,
        center: Entity<PointIn2d>,
        radius: Entity<Distance>,
    ) -> Self {
        Self {
            normal,
            center,
            radius,
        }
    }
}

impl AsEntity for Circle {
    fn type_(&self) -> i32 {
        SLVS_E_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.center.as_handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        Some(self.radius.as_handle())
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
