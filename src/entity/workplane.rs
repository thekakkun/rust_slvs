use super::{AsEntity, Entity, FreeIn3d, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_WORKPLANE},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Workplane {
    pub origin: Entity<Point<FreeIn3d>>,
    pub normal: Entity<Normal<FreeIn3d>>,
}

impl Workplane {
    pub fn new(origin: Entity<Point<FreeIn3d>>, normal: Entity<Normal<FreeIn3d>>) -> Self {
        Self { origin, normal }
    }
}

impl AsEntity for Workplane {
    fn type_(&self) -> i32 {
        SLVS_E_WORKPLANE as _
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.origin.as_handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
