use serde::{Deserialize, Serialize};

use crate::{
    bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    entity::{EntityHandle, PointHandle, SomeEntityHandle},
};
use std::fmt::Debug;

pub trait AsTarget: Copy + Debug + From<Vec<f64>> + Into<Vec<f64>> {
    fn into_some_entity_handle(handle: u32) -> SomeEntityHandle;
    fn slvs_type() -> i32;
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn into_some_entity_handle(h: u32) -> SomeEntityHandle {
        SomeEntityHandle::Point(PointHandle::OnWorkplane(EntityHandle::new(h)))
    }

    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_2D as _
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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn into_some_entity_handle(h: u32) -> SomeEntityHandle {
        SomeEntityHandle::Point(PointHandle::In3d(EntityHandle::new(h)))
    }

    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_3D as _
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
