use serde::{Deserialize, Serialize};

use crate::bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D};
use std::fmt::Debug;

pub trait AsTarget: Copy + Debug + TryFrom<Vec<f64>, Error = &'static str> {
    fn slvs_type() -> i32;
    fn into_vec(self) -> Vec<f64>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_2D as _
    }

    fn into_vec(self) -> Vec<f64> {
        vec![self.0, self.1]
    }
}

impl TryFrom<Vec<f64>> for OnWorkplane {
    type Error = &'static str;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match <[f64; 2]>::try_from(value) {
            Ok(vals) => Ok(Self(vals[0], vals[1])),
            Err(_) => Err("OnWorkplane requires exactly 2 values"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_3D as _
    }

    fn into_vec(self) -> Vec<f64> {
        vec![self.0, self.1, self.2]
    }
}

impl TryFrom<Vec<f64>> for In3d {
    type Error = &'static str;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match <[f64; 3]>::try_from(value) {
            Ok(vals) => Ok(Self(vals[0], vals[1], vals[2])),
            Err(_) => Err("In3d requires exactly 3 values"),
        }
    }
}
