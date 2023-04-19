use super::AsEntity;
use crate::bindings::{Slvs_hEntity, SLVS_E_POINT_IN_3D};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointIn3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PointIn3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl AsEntity for PointIn3d {
    fn type_(&self) -> i32 {
        SLVS_E_POINT_IN_3D as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
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
        Some(vec![self.x, self.y, self.z])
    }
}
