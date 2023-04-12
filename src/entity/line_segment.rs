use crate::binding;

use super::{AsEntity, Entity, PointIn3d};

pub struct LineSegment {
    pub pt_a: Entity<PointIn3d>,
    pub pt_b: Entity<PointIn3d>,
}

impl AsEntity for LineSegment {
    fn type_(&self) -> u32 {
        binding::SLVS_E_LINE_SEGMENT
    }

    fn workplane(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [Some(self.pt_a.into()), Some(self.pt_b.into()), None, None]
    }

    fn normal(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> [Option<f64>; 4] {
        [None; 4]
    }
}
