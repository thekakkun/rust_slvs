use super::AsEntity;
use crate::bindings::{Slvs_hEntity, SLVS_E_DISTANCE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance {
    pub r: f64,
}

impl Distance {
    pub fn new(r: f64) -> Self {
        Self { r }
    }
}

impl AsEntity for Distance {
    fn type_(&self) -> i32 {
        SLVS_E_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
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
        Some(vec![self.r])
    }
}
