use serde::{Deserialize, Serialize};

use crate::bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D};
use std::fmt::Debug;

pub enum Target {
    OnWorkplane,
    In3d,
}

pub trait AsTarget: Copy + Debug + From<Vec<f64>> + Into<Vec<f64>> {
    fn slvs_type() -> i32;
    fn target_type() -> Target;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_2D as _
    }

    fn target_type() -> Target {
        Target::OnWorkplane
    }
}

impl From<Vec<f64>> for OnWorkplane {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1])
    }
}

impl From<OnWorkplane> for Vec<f64> {
    fn from(value: OnWorkplane) -> Self {
        vec![value.0, value.1]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_3D as _
    }

    fn target_type() -> Target {
        Target::In3d
    }
}

impl From<Vec<f64>> for In3d {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1], value[2])
    }
}

impl From<In3d> for Vec<f64> {
    fn from(value: In3d) -> Self {
        vec![value.0, value.1, value.2]
    }
}
