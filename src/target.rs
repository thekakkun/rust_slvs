/*!
Defines where an [entity][`crate::entity`] was sketched.

[`OnWorkplane`] and [`In3d`] also hold coordinate data for points in 2D space and 3D space,
respectively. This is used to ensure that entities in a sketch target ultimately depend on
points sketched within the same dimensionality.
*/

use serde::{Deserialize, Serialize};

use crate::bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D};
use std::fmt::Debug;

/// Things that can be used as a sketch target.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsTarget:
    private::Sealed + Copy + Debug + TryFrom<Vec<f64>, Error = &'static str> + Into<Vec<f64>>
{
    #[doc(hidden)]
    fn slvs_type() -> i32;
}

/// Indicates that an entity is sketched on a workplane.
///
/// This struct is also used to store coordinate data for [`crate::entity::Point`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_2D as _
    }
}

impl From<OnWorkplane> for Vec<f64> {
    fn from(value: OnWorkplane) -> Self {
        vec![value.0, value.1]
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

/// Indicates that an entity is sketched in 3D space.
///
/// This struct is also used to store coordinate data for [`crate::entity::Point`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn slvs_type() -> i32 {
        SLVS_E_POINT_IN_3D as _
    }
}

impl From<In3d> for Vec<f64> {
    fn from(value: In3d) -> Self {
        vec![value.0, value.1, value.2]
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

mod private {
    use super::AsTarget;

    pub trait Sealed {}
    impl<T: AsTarget> Sealed for T {}
}
