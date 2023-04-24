use super::{AsEntityData, Entity, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_WORKPLANE},
    element::{AsHandle, In3d},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Workplane {
    pub origin: Entity<Point<In3d>>,
    pub normal: Entity<Normal<In3d>>,
}

impl Workplane {
    pub fn new(origin: Entity<Point<In3d>>, normal: Entity<Normal<In3d>>) -> Self {
        Self { origin, normal }
    }
}

impl AsEntityData for Workplane {
    type Sketch = In3d;

    fn type_(&self) -> i32 {
        SLVS_E_WORKPLANE as _
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.origin.as_handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }
}
