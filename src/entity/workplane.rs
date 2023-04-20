use super::{AsEntity, Entity, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_WORKPLANE},
    element::{AsHandle, In3D},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Workplane {
    pub origin: Entity<Point<In3D>>,
    pub normal: Entity<Normal<In3D>>,
}

impl Workplane {
    pub fn new(origin: Entity<Point<In3D>>, normal: Entity<Normal<In3D>>) -> Self {
        Self { origin, normal }
    }
}

impl AsEntity for Workplane {
    type Sketch = In3D;

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
