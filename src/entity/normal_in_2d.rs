use super::{AsEntity, Entity, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_NORMAL_IN_2D},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalIn2d {
    pub workplane: Entity<Workplane>,
}

impl NormalIn2d {
    pub fn new(workplane: Entity<Workplane>) -> Self {
        Self { workplane }
    }
}

impl AsEntity for NormalIn2d {
    fn type_(&self) -> i32 {
        SLVS_E_NORMAL_IN_2D as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.as_handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
