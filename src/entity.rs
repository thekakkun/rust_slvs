use std::marker::PhantomData;

use crate::binding;

pub mod line_segment;
pub use line_segment::LineSegment;
pub mod point_in_3d;
pub use point_in_3d::PointIn3d;

////////////////////////////////////////////////////////////////////////////////
// Entity of a specific type
////////////////////////////////////////////////////////////////////////////////

pub trait AsEntity {
    fn type_(&self) -> binding::Slvs_hEntity;
    fn workplane(&self) -> Option<binding::Slvs_hEntity>;
    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4];
    fn normal(&self) -> Option<binding::Slvs_hEntity>;
    fn distance(&self) -> Option<binding::Slvs_hEntity>;
    fn param_vals(&self) -> [Option<f64>; 4];
}

#[derive(Clone, Copy)]
pub struct Entity<T: AsEntity> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsEntity> Entity<T> {
    pub(super) fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
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

////////////////////////////////////////////////////////////////////////////////
// Entity of some sort
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub enum SomeEntity {
    PointIn3d(Entity<PointIn3d>),
    LineSegment(Entity<LineSegment>),
}

impl From<Entity<LineSegment>> for SomeEntity {
    fn from(value: Entity<LineSegment>) -> Self {
        SomeEntity::LineSegment(value)
    }
}

impl From<Entity<PointIn3d>> for SomeEntity {
    fn from(value: Entity<PointIn3d>) -> Self {
        SomeEntity::PointIn3d(value)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity data of some sort
////////////////////////////////////////////////////////////////////////////////

pub enum EntityData {
    PointIn3d(PointIn3d),
    LineSegment(LineSegment),
}

impl From<LineSegment> for EntityData {
    fn from(value: LineSegment) -> Self {
        EntityData::LineSegment(value)
    }
}

impl From<PointIn3d> for EntityData {
    fn from(value: PointIn3d) -> Self {
        EntityData::PointIn3d(value)
    }
}
