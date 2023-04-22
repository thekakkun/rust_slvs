use std::marker::PhantomData;

use super::AsEntity;
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_DISTANCE},
    element::Target,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance<T: Target + ?Sized> {
    pub d: f64,
    phantom: PhantomData<T>,
}

impl<T: Target + ?Sized> Distance<T> {
    pub fn new(d: f64) -> Self {
        Self {
            d,
            phantom: PhantomData,
        }
    }
}

impl<T: Target + ?Sized> AsEntity for Distance<T> {
    type Sketch = T;

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
