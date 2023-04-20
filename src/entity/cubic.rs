use super::{AsEntity, Entity, Point, SketchTarget};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CUBIC},
    AsHandle,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cubic<T: SketchTarget> {
    start_point: Entity<Point<T>>,
    start_control: Entity<Point<T>>,
    end_control: Entity<Point<T>>,
    end_point: Entity<Point<T>>,
}

impl<T: SketchTarget> Cubic<T> {
    pub fn new(
        start_point: Entity<Point<T>>,
        start_control: Entity<Point<T>>,
        end_control: Entity<Point<T>>,
        end_point: Entity<Point<T>>,
    ) -> Self {
        Self {
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }
}

impl<T: SketchTarget> AsEntity for Cubic<T> {
    fn type_(&self) -> i32 {
        SLVS_E_CUBIC as _
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
