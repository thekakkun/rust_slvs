use std::{fmt::Debug, marker::PhantomData};

use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity},
    element::{AsHandle, TypeInfo},
    target::AsTarget,
};

mod point;
pub use point::Point;
mod normal;
pub use normal::Normal;
mod distance;
pub use distance::Distance;
mod workplane;
pub use workplane::Workplane;
mod line_segment;
pub use line_segment::LineSegment;
mod cubic;
pub use cubic::Cubic;
mod circle;
pub use circle::Circle;
mod arc_of_circle;
pub use arc_of_circle::ArcOfCircle;

pub trait AsEntity: AsHandle {}

pub trait AsEntityData: Copy + TypeInfo {
    fn type_(&self) -> i32;
    fn workplane(&self) -> Option<Slvs_hEntity>;

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        None
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

pub trait FromSlvsEntity<T: AsTarget>: AsEntityData {
    fn from(slvs_entity: Slvs_Entity) -> Self;

    fn set_vals(&mut self, _vals: Vec<f64>) {}
}

pub trait As2dProjectionTarget: AsEntityData {}
pub trait AsArc: AsEntityData {}
pub trait AsCubic: AsEntityData {}
pub trait AsCurve: AsEntityData {}
pub trait AsLineSegment: AsEntityData {}
pub trait AsPoint: AsEntityData {}

#[derive(Clone, Copy)]
pub struct Entity<T: AsEntityData> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsEntityData> Entity<T> {
    pub(super) fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsEntityData> Debug for Entity<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity: {{h: {}, type: {}}}", self.handle, T::type_of())
    }
}

impl<T: AsEntityData> AsHandle for Entity<T> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl<T: AsEntityData> AsEntity for Entity<T> {}
