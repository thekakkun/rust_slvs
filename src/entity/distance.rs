use std::marker::PhantomData;

use super::AsEntityData;
use crate::{bindings::SLVS_E_DISTANCE, element::Target};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance<T: Target> {
    pub d: f64,
    phantom: PhantomData<T>,
}

impl<T: Target> Distance<T> {
    pub fn new(d: f64) -> Self {
        Self {
            d,
            phantom: PhantomData,
        }
    }
}

impl<T: Target> AsEntityData for Distance<T> {
    type Sketch = T;

    fn type_(&self) -> i32 {
        SLVS_E_DISTANCE as _
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(vec![self.d])
    }
}
