use super::{AsEntity, Entity, PointIn2d};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CUBIC},
    AsHandle,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cubic {
    pub start_point: Entity<PointIn2d>,
    pub start_control: Entity<PointIn2d>,
    pub end_control: Entity<PointIn2d>,
    pub end_point: Entity<PointIn2d>,
}

impl Cubic {
    pub fn new(
        start_point: Entity<PointIn2d>,
        start_control: Entity<PointIn2d>,
        end_control: Entity<PointIn2d>,
        end_point: Entity<PointIn2d>,
    ) -> Self {
        Self {
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }
}

impl AsEntity for Cubic {
    fn type_(&self) -> i32 {
        SLVS_E_CUBIC as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.start_point.as_handle(),
            self.start_control.as_handle(),
            self.end_control.as_handle(),
            self.end_point.as_handle(),
        ])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
