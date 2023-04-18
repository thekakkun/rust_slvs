use super::AsEntity;
use crate::bindings::{make_quaternion, Slvs_hEntity, SLVS_E_NORMAL_IN_3D};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalIn3d {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl NormalIn3d {
    pub fn new(basis_vec_1: [f64; 3], basic_vec_2: [f64; 3]) -> Self {
        let [w, x, y, z] = make_quaternion(basis_vec_1, basic_vec_2);
        Self { w, x, y, z }
    }
}

impl AsEntity for NormalIn3d {
    fn type_(&self) -> i32 {
        SLVS_E_NORMAL_IN_3D as _
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
        Some(vec![self.w, self.x, self.y, self.z])
    }
}
