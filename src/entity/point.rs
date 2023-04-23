use std::marker::PhantomData;

use super::AsEntityData;
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    element::{In3d, OnWorkplane, Target},
    SomeTarget,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T: Target> {
    pub coords: Coords,
    phantom: PhantomData<T>,
}

impl Point<OnWorkplane> {
    pub fn new(u: f64, v: f64) -> Self {
        Self {
            coords: Coords::OnWorkplane { u, v },
            phantom: PhantomData::<OnWorkplane>,
        }
    }
}

impl Point<In3d> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            coords: Coords::In3d { x, y, z },
            phantom: PhantomData::<In3d>,
        }
    }
}

impl<T: Target> AsEntityData for Point<T> {
    type Sketch = T;

    fn type_(&self) -> i32 {
        match self.coords {
            Coords::OnWorkplane { .. } => SLVS_E_POINT_IN_2D as _,
            Coords::In3d { .. } => SLVS_E_POINT_IN_3D as _,
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
            Coords::OnWorkplane { u, v } => Some(vec![u, v]),
            Coords::In3d { x, y, z } => Some(vec![x, y, z]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coords {
    OnWorkplane { u: f64, v: f64 },
    In3d { x: f64, y: f64, z: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SomePoint;

impl AsEntityData for SomePoint {
    type Sketch = SomeTarget;

    fn type_(&self) -> i32 {
        unimplemented!()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        unimplemented!()
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        unimplemented!()
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        unimplemented!()
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        unimplemented!()
    }
}
