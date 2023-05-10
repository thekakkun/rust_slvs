use serde::{Deserialize, Serialize};
use std::{any::type_name, fmt::Debug, marker::PhantomData};

use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE, SLVS_E_CUBIC,
        SLVS_E_DISTANCE, SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D,
        SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D, SLVS_E_WORKPLANE,
    },
    element::AsHandle,
};

mod point;
pub use point::{Point, PointHandle};
mod normal;
pub use normal::Normal;
mod distance;
pub use distance::{Distance, DistanceHandle};
mod workplane;
pub use workplane::Workplane;
mod line_segment;
pub use line_segment::{LineSegment, LineSegmentHandle};
mod cubic;
pub use cubic::{Cubic, CubicHandle};
mod circle;
pub use circle::{Circle, CircleHandle};
mod arc_of_circle;
pub use arc_of_circle::ArcOfCircle;

pub trait AsEntityData: Debug + Copy {
    fn into_some_entity_handle(handle: u32) -> SomeEntityHandle;
    fn slvs_type(&self) -> i32;
    fn workplane(&self) -> Option<Slvs_hEntity>;
    fn group(&self) -> Slvs_hGroup;

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
    fn set_vals(&mut self, _vals: Vec<f64>) {}
}

pub trait As2dProjectionTarget: AsEntityData {}
pub trait AsArc: AsEntityData {}
pub trait AsCubic: AsEntityData {}
pub trait AsCurve: AsEntityData {}
pub trait AsLineSegment: AsEntityData {}
pub trait AsPoint: AsEntityData {}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct EntityHandle<T: AsEntityData> {
    pub handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<E: AsEntityData> EntityHandle<E> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<E: AsEntityData> Debug for EntityHandle<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Entity: {{h: {}, type: {}}}",
            self.handle,
            type_name::<E>()
        )
    }
}

impl<E: AsEntityData> AsHandle for EntityHandle<E> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl<E: AsEntityData> From<Slvs_Entity> for EntityHandle<E> {
    fn from(value: Slvs_Entity) -> Self {
        EntityHandle::new(value.h)
    }
}

impl<E: AsEntityData> From<SomeEntityHandle> for EntityHandle<E> {
    fn from(value: SomeEntityHandle) -> Self {
        Self::new(value.handle())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SomeEntityHandle {
    ArcOfCircle(EntityHandle<ArcOfCircle>),
    Circle(CircleHandle),
    Cubic(CubicHandle),
    Distance(DistanceHandle),
    LineSegment(LineSegmentHandle),
    Normal(EntityHandle<Normal>),
    Point(PointHandle),
    Workplane(EntityHandle<Workplane>),
}

impl AsHandle for SomeEntityHandle {
    fn handle(&self) -> u32 {
        match self {
            SomeEntityHandle::ArcOfCircle(e) => e.handle(),
            SomeEntityHandle::Circle(e) => e.handle(),
            SomeEntityHandle::Cubic(e) => e.handle(),
            SomeEntityHandle::Distance(e) => e.handle(),
            SomeEntityHandle::LineSegment(e) => e.handle(),
            SomeEntityHandle::Normal(e) => e.handle(),
            SomeEntityHandle::Point(e) => e.handle(),
            SomeEntityHandle::Workplane(e) => e.handle(),
        }
    }
}

impl From<Slvs_Entity> for SomeEntityHandle {
    fn from(value: Slvs_Entity) -> Self {
        match value.type_ as _ {
            SLVS_E_ARC_OF_CIRCLE => SomeEntityHandle::ArcOfCircle(value.into()),
            SLVS_E_CIRCLE => SomeEntityHandle::Circle(value.try_into().unwrap()),
            SLVS_E_CUBIC => SomeEntityHandle::Cubic(value.try_into().unwrap()),
            SLVS_E_DISTANCE => SomeEntityHandle::Distance(value.try_into().unwrap()),
            SLVS_E_LINE_SEGMENT => SomeEntityHandle::LineSegment(value.try_into().unwrap()),
            SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => SomeEntityHandle::Normal(value.into()),
            SLVS_E_POINT_IN_2D | SLVS_E_POINT_IN_3D => {
                SomeEntityHandle::Point(value.try_into().unwrap())
            }
            SLVS_E_WORKPLANE => SomeEntityHandle::Workplane(value.into()),
            _ => panic!("Unknown Slvs_Entity type value {}", value.type_),
        }
    }
}

impl<E: AsEntityData> From<EntityHandle<E>> for SomeEntityHandle {
    fn from(value: EntityHandle<E>) -> Self {
        E::into_some_entity_handle(value.handle())
    }
}
