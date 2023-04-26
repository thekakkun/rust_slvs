use std::marker::PhantomData;

use super::{AsEntityData, AsPoint, Entity, FromSlvsEntity, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    element::{AsHandle, AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coords {
    OnWorkplane { u: f64, v: f64 },
    In3d { x: f64, y: f64, z: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T: AsTarget> {
    pub workplane: Option<Entity<Workplane>>,
    pub coords: Coords,
    phantom: PhantomData<T>,
}

impl Point<OnWorkplane> {
    pub fn new(workplane: Entity<Workplane>, u: f64, v: f64) -> Self {
        Self {
            workplane: Some(workplane),
            coords: Coords::OnWorkplane { u, v },
            phantom: PhantomData::<OnWorkplane>,
        }
    }
}

impl Point<In3d> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            workplane: None,
            coords: Coords::In3d { x, y, z },
            phantom: PhantomData::<In3d>,
        }
    }
}

impl<T: AsTarget> AsPoint for Point<T> {}

impl<T: AsTarget> AsEntityData for Point<T> {
    fn type_(&self) -> i32 {
        match self.coords {
            Coords::OnWorkplane { .. } => SLVS_E_POINT_IN_2D as _,
            Coords::In3d { .. } => SLVS_E_POINT_IN_3D as _,
        }
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.as_handle())
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        match self.coords {
            Coords::OnWorkplane { u, v } => Some(vec![u, v]),
            Coords::In3d { x, y, z } => Some(vec![x, y, z]),
        }
    }
}

impl FromSlvsEntity<OnWorkplane> for Point<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            coords: Coords::OnWorkplane { u: 0.0, v: 0.0 },
            phantom: PhantomData,
        }
    }

    fn set_vals(&mut self, params: Vec<f64>) {
        self.coords = Coords::OnWorkplane {
            u: params[0],
            v: params[1],
        }
    }
}

impl FromSlvsEntity<In3d> for Point<In3d> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            coords: Coords::In3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            phantom: PhantomData,
        }
    }

    fn set_vals(&mut self, params: Vec<f64>) {
        self.coords = Coords::In3d {
            x: params[0],
            y: params[1],
            z: params[2],
        }
    }
}
