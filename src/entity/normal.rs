use std::marker::PhantomData;

use super::{AsEntity, Entity, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D},
    element::{In3d, OnWorkplane, Target},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normal<T: Target> {
    data: NormalDef,
    phantom: PhantomData<T>,
}

impl Normal<OnWorkplane> {
    pub fn new(workplane: Entity<Workplane>) -> Self {
        Self {
            data: NormalDef::OnWorkplane { workplane },
            phantom: PhantomData::<OnWorkplane>,
        }
    }
}

impl Normal<In3d> {
    pub fn new(quaternion: [f64; 4]) -> Self {
        let [w, x, y, z] = quaternion;
        Self {
            data: NormalDef::In3d { w, x, y, z },
            phantom: PhantomData::<In3d>,
        }
    }
}

impl<T: Target> AsEntity for Normal<T> {
    type Sketch = T;

    fn type_(&self) -> i32 {
        match self.data {
            NormalDef::OnWorkplane { .. } => SLVS_E_NORMAL_IN_2D as _,
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
            NormalDef::OnWorkplane { .. } => None,
            NormalDef::In3d { w, x, y, z } => Some(vec![w, x, y, z]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NormalDef {
    OnWorkplane { workplane: Entity<Workplane> },
    In3d { w: f64, x: f64, y: f64, z: f64 },
}
