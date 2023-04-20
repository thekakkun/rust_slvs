use std::marker::PhantomData;

use super::{AsEntity, SketchTarget};
use crate::bindings::{Slvs_hEntity, SLVS_E_DISTANCE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance<T: SketchTarget> {
    pub d: f64,
    phantom: PhantomData<T>,
}

impl<T: SketchTarget> Distance<T> {
    pub fn new(d: f64) -> Self {
        Self {
            d,
            phantom: PhantomData,
        }
    }
}

impl<T: SketchTarget> AsEntity for Distance<T> {
    type SketchedOn = T;

    fn type_(&self) -> i32 {
        SLVS_E_DISTANCE as _
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
        Some(vec![self.d])
    }
}
