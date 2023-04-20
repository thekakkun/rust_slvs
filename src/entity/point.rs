use std::marker::PhantomData;

use super::{AsEntity, FreeIn3d, OnWorkplane, SketchTarget};
use crate::bindings::{Slvs_hEntity, SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T: SketchTarget> {
    pub coords: PointCoords,
    phantom: PhantomData<T>,
}

impl Point<OnWorkplane> {
    fn new(u: f64, v: f64) -> Self {
        Self {
            coords: PointCoords::In2d { u, v },
            phantom: PhantomData::<OnWorkplane>,
        }
    }
}

impl Point<FreeIn3d> {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            coords: PointCoords::In3d { x, y, z },
            phantom: PhantomData::<FreeIn3d>,
        }
    }
}

impl<T: SketchTarget> AsEntity for Point<T> {
    fn type_(&self) -> i32 {
        match self.coords {
            PointCoords::In2d { .. } => SLVS_E_POINT_IN_2D as _,
            PointCoords::In3d { .. } => SLVS_E_POINT_IN_3D as _,
        }
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
        match self.coords {
            PointCoords::In2d { u, v } => Some(vec![u, v]),
            PointCoords::In3d { x, y, z } => Some(vec![x, y, z]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PointCoords {
    In2d { u: f64, v: f64 },
    In3d { x: f64, y: f64, z: f64 },
}
