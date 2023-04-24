use super::{AsEntityData, Distance, Entity, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CIRCLE},
    element::{AsHandle, AsTarget},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle<T: AsTarget> {
    center: Entity<Point<T>>,
    radius: Entity<Distance<T>>,
    normal: Entity<Normal<T>>,
}

impl<T: AsTarget> Circle<T> {
    pub fn new(
        center: Entity<Point<T>>,
        radius: Entity<Distance<T>>,
        normal: Entity<Normal<T>>,
    ) -> Self {
        Self {
            center,
            radius,
            normal,
        }
    }
}

impl<T: AsTarget> AsEntityData for Circle<T> {
    type Sketch = T;

    fn type_(&self) -> i32 {
        SLVS_E_CIRCLE as _
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.center.as_handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        Some(self.radius.as_handle())
    }
}
