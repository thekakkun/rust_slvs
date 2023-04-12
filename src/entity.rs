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

impl<T: AsEntity> From<Entity<T>> for binding::Slvs_hEntity {
    fn from(value: Entity<T>) -> Self {
        value.handle
    }
}

// impl<T: AsEntity> From<binding::Slvs_Entity> for Entity<T> {
//     fn from(value: binding::Slvs_Entity) -> Self {
//         match value.type_ {

//         }
//     }
// }

pub enum EntityType {
    PointIn3d = binding::SLVS_E_POINT_IN_3D as _,
    PointIn2d = binding::SLVS_E_POINT_IN_2D as _,
    NormalIn3d = binding::SLVS_E_NORMAL_IN_3D as _,
    NormalIn2d = binding::SLVS_E_NORMAL_IN_2D as _,
    Distance = binding::SLVS_E_DISTANCE as _,
    Workplane = binding::SLVS_E_WORKPLANE as _,
    LineSegment = binding::SLVS_E_LINE_SEGMENT as _,
    Cubic = binding::SLVS_E_CUBIC as _,
    Circle = binding::SLVS_E_CIRCLE as _,
    ArcOfCircle = binding::SLVS_E_ARC_OF_CIRCLE as _,
}

pub trait AsEntity {
    fn type_(&self) -> EntityType;
    fn workplane(&self) -> Option<binding::Slvs_hEntity>;
    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4];
    fn normal(&self) -> Option<binding::Slvs_hEntity>;
    fn distance(&self) -> Option<binding::Slvs_hEntity>;
    fn param_vals(&self) -> [Option<f64>; 4];
}
