use std::marker::PhantomData;

use super::{AsEntityData, AsNormal, Entity, FromSlvsEntity, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D},
    element::{AsHandle, AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NormalDef {
    OnWorkplane { workplane: Entity<Workplane> },
    In3d { w: f64, x: f64, y: f64, z: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normal<T: AsTarget> {
    pub data: NormalDef,
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

impl<T: AsTarget> AsNormal for Normal<T> {}

impl<T: AsTarget> AsEntityData for Normal<T> {
    fn type_(&self) -> i32 {
        match self.data {
            NormalDef::OnWorkplane { .. } => SLVS_E_NORMAL_IN_2D as _,
            NormalDef::In3d { .. } => SLVS_E_NORMAL_IN_3D as _,
        }
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self.data {
            NormalDef::OnWorkplane { workplane } => Some(workplane.as_handle()),
            NormalDef::In3d { .. } => None,
        }
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        match self.data {
            NormalDef::OnWorkplane { .. } => None,
            NormalDef::In3d { w, x, y, z } => Some(vec![w, x, y, z]),
        }
    }
}

impl FromSlvsEntity<OnWorkplane> for Normal<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            data: NormalDef::OnWorkplane {
                workplane: Entity::new(slvs_entity.wrkpl),
            },
            phantom: PhantomData,
        }
    }
}

impl FromSlvsEntity<In3d> for Normal<In3d> {
    fn from(_slvs_entity: Slvs_Entity) -> Self {
        Self {
            data: NormalDef::In3d {
                w: 0.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            phantom: PhantomData,
        }
    }

    fn set_vals(&mut self, vals: Vec<f64>) {
        self.data = NormalDef::In3d {
            w: vals[0],
            x: vals[1],
            y: vals[2],
            z: vals[3],
        }
    }
}
