use super::{AsEntity, Distance, Entity, Normal, Point, SketchTarget};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CIRCLE},
    element::AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle<T: SketchTarget> {
    center: Entity<Point<T>>,
    radius: Entity<Distance<T>>,
    normal: Entity<Normal<T>>,
}

impl<T: SketchTarget> Circle<T> {
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

impl<T: SketchTarget> AsEntity for Circle<T> {
    type SketchedOn = T;

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

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
