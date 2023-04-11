use crate::binding;

use super::{AsEntity, Entity, EntityType};

pub struct LineSegment {
    pub pt_a: Entity,
    pub pt_b: Entity,
}

impl AsEntity for LineSegment {
    fn type_(&self) -> EntityType {
        EntityType::LineSegment
    }

    fn wrkpl(&self) -> Option<binding::Slvs_hEntity> {
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
