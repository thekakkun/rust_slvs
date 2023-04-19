use super::{AsEntity, Entity, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_DISTANCE},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance {
    pub workplane: Entity<Workplane>,
    pub d: f64,
}

impl Distance {
    pub fn new(workplane: Entity<Workplane>, d: f64) -> Self {
        Self { workplane, d }
    }
}

impl AsEntity for Distance {
    fn type_(&self) -> i32 {
        SLVS_E_DISTANCE as _
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
        Some(vec![self.d])
    }
}
