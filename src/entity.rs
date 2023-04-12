use std::marker::PhantomData;

use crate::binding;

pub mod line_segment;
pub use line_segment::LineSegment;
pub mod point_in_3d;
pub use point_in_3d::PointIn3d;

#[derive(Clone, Copy)]
pub struct Entity<T: AsEntity + ?Sized> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsEntity> Entity<T> {
    pub fn from(handle: u32) -> Self {
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

pub trait AsEntity {
    fn type_(&self) -> u32;
    fn workplane(&self) -> Option<binding::Slvs_hEntity>;
    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4];
    fn normal(&self) -> Option<binding::Slvs_hEntity>;
    fn distance(&self) -> Option<binding::Slvs_hEntity>;
    fn param_vals(&self) -> [Option<f64>; 4];
}
