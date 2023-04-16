use crate::bindings;

use super::{AsEntity, Entity, PointIn3d};

#[derive(Clone, Copy)]
pub struct LineSegment {
    pub point_a: Entity<PointIn3d>,
    pub point_b: Entity<PointIn3d>,
}

impl AsEntity for LineSegment {
    fn type_(&self) -> i32 {
        bindings::SLVS_E_LINE_SEGMENT as _
    }

    fn workplane(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<bindings::Slvs_hEntity>; 4] {
        [
            Some(self.point_a.into()),
            Some(self.point_b.into()),
            None,
            None,
        ]
    }

    fn normal(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
