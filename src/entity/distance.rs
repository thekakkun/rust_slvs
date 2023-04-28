use std::marker::PhantomData;

use super::{AsEntityData, Entity, FromSlvsEntity, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_DISTANCE},
    element::{AsHandle, TypeInfo},
    target::{AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug)]
pub struct Distance<T: AsTarget> {
    pub workplane: Option<Entity<Workplane>>,
    pub val: f64,
    phantom: PhantomData<T>,
}

impl Distance<OnWorkplane> {
    pub fn new(workplane: Entity<Workplane>, val: f64) -> Self {
        Self {
            workplane: Some(workplane),
            val,
            phantom: PhantomData,
        }
    }
}

impl Distance<In3d> {
    pub fn new(val: f64) -> Self {
        Self {
            workplane: None,
            val,
            phantom: PhantomData,
        }
    }
}

impl<T: AsTarget> AsEntityData for Distance<T> {
    fn type_(&self) -> i32 {
        SLVS_E_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(vec![self.val])
    }
}

impl<T: AsTarget> TypeInfo for Distance<T> {
    fn type_of() -> String {
        format!("Distance<{}>", T::type_of())
    }
}

impl FromSlvsEntity<OnWorkplane> for Distance<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            val: 0.0,
            phantom: PhantomData,
        }
    }

    fn set_vals(&mut self, vals: Vec<f64>) {
        self.val = vals[0]
    }
}

impl FromSlvsEntity<In3d> for Distance<In3d> {
    fn from(_slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: None,
            val: 0.0,
            phantom: PhantomData,
        }
    }

    fn set_vals(&mut self, vals: Vec<f64>) {
        self.val = vals[0]
    }
}
