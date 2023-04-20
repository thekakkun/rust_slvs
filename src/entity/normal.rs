use std::marker::PhantomData;

use super::{AsEntity, Entity, FreeIn3d, OnWorkplane, SketchTarget, Workplane};
use crate::bindings::{Slvs_hEntity, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normal<T: SketchTarget> {
    data: NormalDef,
    phantom: PhantomData<T>,
}

impl Normal<OnWorkplane> {
    pub fn new(workplane: Entity<Workplane>) -> Self {
        Self {
            data: NormalDef::In2d { workplane },
            phantom: PhantomData::<OnWorkplane>,
        }
    }
}

impl Normal<FreeIn3d> {
    pub fn new(quaternion: [f64; 4]) -> Self {
        let [w, x, y, z] = quaternion;
        Self {
            data: NormalDef::In3d { w, x, y, z },
            phantom: PhantomData::<FreeIn3d>,
        }
    }
}

impl<T: SketchTarget> AsEntity for Normal<T> {
    type SketchedOn = T;

    fn type_(&self) -> i32 {
        match self.data {
            NormalDef::In2d { .. } => SLVS_E_NORMAL_IN_2D as _,
            NormalDef::In3d { .. } => SLVS_E_NORMAL_IN_3D as _,
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
        match self.data {
            NormalDef::In2d { .. } => None,
            NormalDef::In3d { w, x, y, z } => Some(vec![w, x, y, z]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum NormalDef {
    In2d { workplane: Entity<Workplane> },
    In3d { w: f64, x: f64, y: f64, z: f64 },
}
