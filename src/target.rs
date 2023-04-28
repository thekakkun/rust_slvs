use crate::{
    bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    element::TypeInfo,
};
use std::fmt::Debug;

pub trait AsTarget: Copy + TypeInfo {
    fn type_() -> i32;
    fn as_vec(&self) -> Vec<f64>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_2D as _
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.0, self.1]
    }
}

impl TypeInfo for OnWorkplane {
    fn type_of() -> String {
        "OnWorkplane".to_string()
    }
}

impl From<Vec<f64>> for OnWorkplane {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_3D as _
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.0, self.1, self.2]
    }
}

impl TypeInfo for In3d {
    fn type_of() -> String {
        "In3d".to_string()
    }
}

impl From<Vec<f64>> for In3d {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1], value[2])
    }
}
