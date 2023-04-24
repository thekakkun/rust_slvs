use super::{AsEntityData, Entity, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CUBIC},
    element::{AsHandle, AsTarget},
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cubic<T: AsTarget> {
    start_point: Entity<Point<T>>,
    start_control: Entity<Point<T>>,
    end_control: Entity<Point<T>>,
    end_point: Entity<Point<T>>,
}

impl<T: AsTarget> Cubic<T> {
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

impl<T: AsTarget> AsEntityData for Cubic<T> {
    type Sketch = T;

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
}
