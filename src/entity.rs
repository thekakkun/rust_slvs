use serde::{Deserialize, Serialize};
use std::{any::type_name, fmt::Debug, marker::PhantomData};

use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE, SLVS_E_CUBIC,
        SLVS_E_DISTANCE, SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D,
        SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D, SLVS_E_WORKPLANE,
    },
    element::AsHandle,
    target::{In3d, OnWorkplane},
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

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl<E: AsEntityData> TryFrom<SomeEntityHandle> for EntityHandle<E> {
    type Error = &'static str;

    fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
        let entity_handle = Self::new(value.handle());

        if value == entity_handle.into() {
            Ok(entity_handle)
        } else {
            Err("Not of expected type")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SomeEntityHandle {
    ArcOfCircle(EntityHandle<ArcOfCircle>),
    CircleOnWorkplane(EntityHandle<Circle<OnWorkplane>>),
    CircleIn3d(EntityHandle<Circle<In3d>>),
    CubicOnWorkplane(EntityHandle<Cubic<OnWorkplane>>),
    CubicIn3d(EntityHandle<Cubic<In3d>>),
    DistanceOnWorkplane(EntityHandle<Distance<OnWorkplane>>),
    DistanceIn3d(EntityHandle<Distance<In3d>>),
    LineSegmentOnWorkplane(EntityHandle<LineSegment<OnWorkplane>>),
    LineSegmentIn3d(EntityHandle<LineSegment<In3d>>),
    Normal(EntityHandle<Normal>),
    PointOnWorkplane(EntityHandle<Point<OnWorkplane>>),
    PointIn3d(EntityHandle<Point<In3d>>),
    Workplane(EntityHandle<Workplane>),
}

impl AsHandle for SomeEntityHandle {
    fn handle(&self) -> u32 {
        match self {
            SomeEntityHandle::ArcOfCircle(e) => e.handle(),
            SomeEntityHandle::CircleOnWorkplane(e) => e.handle(),
            SomeEntityHandle::CircleIn3d(e) => e.handle(),
            SomeEntityHandle::CubicOnWorkplane(e) => e.handle(),
            SomeEntityHandle::CubicIn3d(e) => e.handle(),
            SomeEntityHandle::DistanceOnWorkplane(e) => e.handle(),
            SomeEntityHandle::DistanceIn3d(e) => e.handle(),
            SomeEntityHandle::LineSegmentOnWorkplane(e) => e.handle(),
            SomeEntityHandle::LineSegmentIn3d(e) => e.handle(),
            SomeEntityHandle::Normal(e) => e.handle(),
            SomeEntityHandle::PointOnWorkplane(e) => e.handle(),
            SomeEntityHandle::PointIn3d(e) => e.handle(),
            SomeEntityHandle::Workplane(e) => e.handle(),
        }
    }
}

impl From<Slvs_Entity> for SomeEntityHandle {
    fn from(value: Slvs_Entity) -> Self {
        match value.type_ as _ {
            SLVS_E_ARC_OF_CIRCLE => SomeEntityHandle::ArcOfCircle(value.into()),
            SLVS_E_CIRCLE => match value.wrkpl {
                0 => SomeEntityHandle::CircleIn3d(value.try_into().unwrap()),
                _ => SomeEntityHandle::CircleOnWorkplane(value.try_into().unwrap()),
            },
            SLVS_E_CUBIC => match value.wrkpl {
                0 => SomeEntityHandle::CubicIn3d(value.try_into().unwrap()),
                _ => SomeEntityHandle::CubicOnWorkplane(value.try_into().unwrap()),
            },
            SLVS_E_DISTANCE => match value.wrkpl {
                0 => SomeEntityHandle::DistanceIn3d(value.try_into().unwrap()),
                _ => SomeEntityHandle::DistanceOnWorkplane(value.try_into().unwrap()),
            },
            SLVS_E_LINE_SEGMENT => match value.wrkpl {
                0 => SomeEntityHandle::LineSegmentIn3d(value.try_into().unwrap()),
                _ => SomeEntityHandle::LineSegmentOnWorkplane(value.try_into().unwrap()),
            },
            SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => SomeEntityHandle::Normal(value.into()),
            SLVS_E_POINT_IN_2D => SomeEntityHandle::PointOnWorkplane(value.try_into().unwrap()),
            SLVS_E_POINT_IN_3D => SomeEntityHandle::PointIn3d(value.try_into().unwrap()),
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
