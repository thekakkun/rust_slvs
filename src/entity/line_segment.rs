use crate::binding;

use super::{AsEntity, Entity, PointIn3d, SomeEntity};

pub struct LineSegment {
    pub point_a: Entity<PointIn3d>,
    pub point_b: Entity<PointIn3d>,
}

impl AsEntity for LineSegment {
    fn type_(&self) -> binding::Slvs_hEntity {
        binding::SLVS_E_LINE_SEGMENT
    }

    fn workplane(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [
            Some(self.point_a.into()),
            Some(self.point_b.into()),
            None,
            None,
        ]
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

impl TryFrom<SomeEntity> for Entity<LineSegment> {
    type Error = &'static str;

    fn try_from(value: SomeEntity) -> Result<Self, Self::Error> {
        if let SomeEntity::LineSegment(entity) = value {
            Ok(entity)
        } else {
            Err("Expected SomeEntity::LineSegment")
        }
    }
}

impl From<Entity<LineSegment>> for SomeEntity {
    fn from(value: Entity<LineSegment>) -> Self {
        SomeEntity::LineSegment(value)
    }
}