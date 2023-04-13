use std::marker::PhantomData;

use crate::binding;

pub mod line_segment;
pub use line_segment::LineSegment;
pub mod point_in_3d;
pub use point_in_3d::PointIn3d;

pub trait AsEntity {
    fn type_(&self) -> binding::Slvs_hEntity;
    fn workplane(&self) -> Option<binding::Slvs_hEntity>;
    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4];
    fn normal(&self) -> Option<binding::Slvs_hEntity>;
    fn distance(&self) -> Option<binding::Slvs_hEntity>;
    fn param_vals(&self) -> [Option<f64>; 4];
}

#[derive(Clone, Copy)]
pub struct Entity<T: AsEntity + ?Sized> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsEntity + ?Sized> Entity<T> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsEntity> From<Entity<T>> for binding::Slvs_hEntity {
    fn from(value: Entity<T>) -> Self {
        value.handle
    }
}

pub enum SomeEntity {
    PointIn3d(Entity<PointIn3d>),
    LineSegment(Entity<LineSegment>),
}
