use std::marker::PhantomData;

use super::{AsEntityData, Entity, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_DISTANCE},
    element::{AsHandle, AsTarget},
    In3d, OnWorkplane,
};

#[derive(Clone, Copy, Debug, PartialEq)]
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
        self.workplane.map(|workplane| workplane.as_handle())
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(vec![self.val])
    }
}
