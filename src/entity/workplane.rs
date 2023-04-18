use super::{AsEntity, Entity, NormalIn3d, PointIn3d};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_WORKPLANE},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Workplane {
    pub origin: Entity<PointIn3d>,
    pub normal: Entity<NormalIn3d>,
}

impl Workplane {
    pub fn new(origin: Entity<PointIn3d>, normal: Entity<NormalIn3d>) -> Self {
        Self { origin, normal }
    }
}

impl AsEntity for Workplane {
    fn type_(&self) -> i32 {
        SLVS_E_WORKPLANE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
