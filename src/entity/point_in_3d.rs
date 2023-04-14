use crate::binding;

use super::{AsEntity, Entity, SomeEntity, EntityData};

#[derive(Clone, Copy)]
pub struct PointIn3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl AsEntity for PointIn3d {
    fn type_(&self) -> binding::Slvs_hEntity {
        binding::SLVS_E_POINT_IN_3D
    }

    fn workplane(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [None; 4]
    }

    fn normal(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> [Option<f64>; 4] {
        [Some(self.x), Some(self.y), Some(self.z), None]
    }
}

////////////////////////////////////////////////////////////////////////////////
// Conversions between SomeEntity
////////////////////////////////////////////////////////////////////////////////

impl TryFrom<SomeEntity> for Entity<PointIn3d> {
    type Error = &'static str;

    fn try_from(value: SomeEntity) -> Result<Self, Self::Error> {
        if let SomeEntity::PointIn3d(entity) = value {
            Ok(entity)
        } else {
            Err("Expected SomeEntity::PointIn3d")
        }
    }
}

impl From<Entity<PointIn3d>> for SomeEntity {
    fn from(value: Entity<PointIn3d>) -> Self {
        SomeEntity::PointIn3d(value)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Conversions between EntityData
////////////////////////////////////////////////////////////////////////////////\

impl From<PointIn3d> for EntityData {
    fn from(value: PointIn3d) -> Self {
        EntityData::PointIn3d(value)
    }
}

impl TryFrom<EntityData> for PointIn3d {
    type Error = &'static str;

    fn try_from(value: EntityData) -> Result<Self, Self::Error> {
        if let EntityData::PointIn3d(data) = value {
            Ok(data)
        } else {
            Err("Expected EntityData::PointIn3d")
        }
    }
}
