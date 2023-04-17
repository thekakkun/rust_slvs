use super::{AsEntity, Entity, PointIn3d};
use crate::{Slvs_hEntity, SLVS_E_LINE_SEGMENT};

#[derive(Clone, Copy)]
pub struct LineSegment {
    pub point_a: Entity<PointIn3d>,
    pub point_b: Entity<PointIn3d>,
}

impl LineSegment {
    pub fn new(point_a: Entity<PointIn3d>, point_b: Entity<PointIn3d>) -> Self {
        Self { point_a, point_b }
    }
}

impl AsEntity for LineSegment {
    fn type_(&self) -> i32 {
        SLVS_E_LINE_SEGMENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.into(), self.point_b.into()])
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
