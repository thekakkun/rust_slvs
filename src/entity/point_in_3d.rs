use crate::bindings;

use super::AsEntity;

#[derive(Clone, Copy)]
pub struct PointIn3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl AsEntity for PointIn3d {
    fn type_(&self) -> bindings::Slvs_hEntity {
        bindings::SLVS_E_POINT_IN_3D
    }

    fn workplane(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<bindings::Slvs_hEntity>; 4] {
        [None; 4]
    }

    fn normal(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> [Option<f64>; 4] {
        [Some(self.x), Some(self.y), Some(self.z), None]
    }
}
