use std::marker::PhantomData;

use super::AsEntityData;
use crate::{bindings::SLVS_E_DISTANCE, element::AsTarget};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance<T: AsTarget> {
    pub d: f64,
    phantom: PhantomData<T>,
}

impl<T: AsTarget> Distance<T> {
    pub fn new(d: f64) -> Self {
        Self {
            d,
            phantom: PhantomData,
        }
    }
}

impl<T: AsTarget> AsEntityData for Distance<T> {
    type Sketch = T;

    fn type_(&self) -> i32 {
        SLVS_E_DISTANCE as _
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(vec![self.d])
    }
}
