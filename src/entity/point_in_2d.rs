use super::{AsEntity, Entity, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_POINT_IN_2D},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointIn2d {
    pub workplane: Entity<Workplane>,
    pub u: f64,
    pub v: f64,
}

impl PointIn2d {
    pub fn new(workplane: Entity<Workplane>, u: f64, v: f64) -> Self {
        Self { workplane, u, v }
    }
}

impl AsEntity for PointIn2d {
    fn type_(&self) -> i32 {
        SLVS_E_POINT_IN_2D as _
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
        Some(vec![self.u, self.v])
    }
}
