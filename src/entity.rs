pub use arc_of_circle::ArcOfCircle;
pub use circle::Circle;
pub use cubic::Cubic;
pub use distance::Distance;
pub use line_segment::LineSegment;
pub use normal::Normal;
pub use point::Point;
pub use workplane::Workplane;

mod arc_of_circle;
mod circle;
mod cubic;
mod distance;
mod line_segment;
mod normal;
mod point;
mod workplane;

use std::{any::type_name, fmt::Debug, marker::PhantomData};

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE, SLVS_E_CUBIC,
        SLVS_E_DISTANCE, SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D,
        SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D, SLVS_E_WORKPLANE,
    },
    element::AsHandle,
    target::{In3d, OnWorkplane},
    System,
};

////////////////////////////////////////////////////////////////////////////////
// Entity Handle
////////////////////////////////////////////////////////////////////////////////

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

impl<E: AsEntityData> AsHandle for EntityHandle<E> {
    fn handle(&self) -> u32 {
        self.handle
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

impl<E: AsEntityData> From<Slvs_Entity> for EntityHandle<E> {
    fn from(value: Slvs_Entity) -> Self {
        EntityHandle::new(value.h)
    }
}

#[enum_dispatch(SomeEntityHandle)]
pub trait AsEntityHandle {}
impl<E: AsEntityData> AsEntityHandle for EntityHandle<E> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
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

impl From<LineSegmentHandle> for SomeEntityHandle {
    fn from(value: LineSegmentHandle) -> Self {
        match value {
            LineSegmentHandle::OnWorkplane(h) => Self::LineSegmentOnWorkplane(h),
            LineSegmentHandle::In3d(h) => Self::LineSegmentIn3d(h),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity handle for some arc
////////////////////////////////////////////////////////////////////////////////

pub trait AsArc: AsEntityData {}

#[enum_dispatch(ArcHandle)]
trait AsArcHandle {}
impl AsArcHandle for EntityHandle<ArcOfCircle> {}
impl AsArcHandle for EntityHandle<Circle<OnWorkplane>> {}
impl AsArcHandle for EntityHandle<Circle<In3d>> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
pub enum ArcHandle {
    ArcOfCircle(EntityHandle<ArcOfCircle>),
    CircleOnWorkplane(EntityHandle<Circle<OnWorkplane>>),
    CircleIn3d(EntityHandle<Circle<In3d>>),
}

impl TryFrom<Slvs_Entity> for ArcHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        match value.type_ as _ {
            SLVS_E_ARC_OF_CIRCLE => Ok(ArcHandle::ArcOfCircle(value.into())),
            SLVS_E_CIRCLE => match value.wrkpl {
                0 => Ok(ArcHandle::CircleIn3d(value.into())),
                _ => Ok(ArcHandle::CircleOnWorkplane(value.into())),
            },
            _ => Err("Expected Slvs_Entity type of arc or circle"),
        }
    }
}

impl TryFrom<SomeEntityHandle> for ArcHandle {
    type Error = &'static str;

    fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
        match value {
            SomeEntityHandle::ArcOfCircle(h) => Ok(ArcHandle::ArcOfCircle(h)),
            SomeEntityHandle::CircleOnWorkplane(h) => Ok(ArcHandle::CircleOnWorkplane(h)),
            SomeEntityHandle::CircleIn3d(h) => Ok(ArcHandle::CircleIn3d(h)),
            _ => Err("Expected Arc or Circle variant of SomeEntityHandle"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity handle for some cubic
////////////////////////////////////////////////////////////////////////////////

pub trait AsCubic: AsEntityData {}

#[enum_dispatch(CubicHandle)]
trait AsCubicHandle {}
impl AsCubicHandle for EntityHandle<Cubic<OnWorkplane>> {}
impl AsCubicHandle for EntityHandle<Cubic<In3d>> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
pub enum CubicHandle {
    OnWorkplane(EntityHandle<Cubic<OnWorkplane>>),
    In3d(EntityHandle<Cubic<In3d>>),
}

impl TryFrom<Slvs_Entity> for CubicHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        match value.type_ as _ {
            SLVS_E_CUBIC => match value.wrkpl {
                0 => Ok(CubicHandle::In3d(value.into())),
                _ => Ok(CubicHandle::OnWorkplane(value.into())),
            },
            _ => Err("Expected Slvs_Entity type of cubic"),
        }
    }
}

impl TryFrom<SomeEntityHandle> for CubicHandle {
    type Error = &'static str;

    fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
        match value {
            SomeEntityHandle::CubicOnWorkplane(h) => Ok(CubicHandle::OnWorkplane(h)),
            SomeEntityHandle::CubicIn3d(h) => Ok(CubicHandle::In3d(h)),
            _ => Err("Expected Cubic variant of SomeEntityHandle"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity handle for some curve
////////////////////////////////////////////////////////////////////////////////

pub trait AsCurve: AsEntityData {}

#[enum_dispatch(CurveHandle)]
trait AsCurveHandle {}
impl AsCurveHandle for EntityHandle<ArcOfCircle> {}
impl AsCurveHandle for EntityHandle<Cubic<OnWorkplane>> {}
impl AsCurveHandle for EntityHandle<Cubic<In3d>> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
pub enum CurveHandle {
    ArcOfCircle(EntityHandle<ArcOfCircle>),
    CubicOnWorkplane(EntityHandle<Cubic<OnWorkplane>>),
    CubicIn3d(EntityHandle<Cubic<In3d>>),
}

impl TryFrom<Slvs_Entity> for CurveHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        match value.type_ as _ {
            SLVS_E_ARC_OF_CIRCLE => Ok(CurveHandle::ArcOfCircle(value.into())),
            SLVS_E_CUBIC => match value.wrkpl {
                0 => Ok(CurveHandle::CubicIn3d(value.into())),
                _ => Ok(CurveHandle::CubicOnWorkplane(value.into())),
            },
            _ => Err("Expected Slvs_Entity type of arc or cubic"),
        }
    }
}

impl TryFrom<SomeEntityHandle> for CurveHandle {
    type Error = &'static str;

    fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
        match value {
            SomeEntityHandle::ArcOfCircle(h) => Ok(CurveHandle::ArcOfCircle(h)),
            SomeEntityHandle::CubicOnWorkplane(h) => Ok(CurveHandle::CubicOnWorkplane(h)),
            SomeEntityHandle::CubicIn3d(h) => Ok(CurveHandle::CubicIn3d(h)),
            _ => Err("Expected Arc or Cubic variant of SomeEntityHandle"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity handle for some line
////////////////////////////////////////////////////////////////////////////////

pub trait AsLineSegment: AsEntityData {}

#[enum_dispatch(LineSegmentHandle)]
trait AsLineSegmentHandle {}
impl AsLineSegmentHandle for EntityHandle<LineSegment<OnWorkplane>> {}
impl AsLineSegmentHandle for EntityHandle<LineSegment<In3d>> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
pub enum LineSegmentHandle {
    OnWorkplane(EntityHandle<LineSegment<OnWorkplane>>),
    In3d(EntityHandle<LineSegment<In3d>>),
}

impl TryFrom<Slvs_Entity> for LineSegmentHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        match value.type_ as _ {
            SLVS_E_LINE_SEGMENT => match value.wrkpl {
                0 => Ok(LineSegmentHandle::In3d(value.into())),
                _ => Ok(LineSegmentHandle::OnWorkplane(value.into())),
            },
            _ => Err("Expected Slvs_Entity type of line segment"),
        }
    }
}

impl TryFrom<SomeEntityHandle> for LineSegmentHandle {
    type Error = &'static str;

    fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
        match value {
            SomeEntityHandle::LineSegmentOnWorkplane(h) => Ok(LineSegmentHandle::OnWorkplane(h)),
            SomeEntityHandle::LineSegmentIn3d(h) => Ok(LineSegmentHandle::In3d(h)),
            _ => Err("Expected LineSegment variant of SomeEntityHandle"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity handle for some point
////////////////////////////////////////////////////////////////////////////////

pub trait AsPoint: AsEntityData {}

#[enum_dispatch(PointHandle)]
trait AsPointHandle {}
impl AsPointHandle for EntityHandle<Point<OnWorkplane>> {}
impl AsPointHandle for EntityHandle<Point<In3d>> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
pub enum PointHandle {
    OnWorkplane(EntityHandle<Point<OnWorkplane>>),
    In3d(EntityHandle<Point<In3d>>),
}

impl TryFrom<Slvs_Entity> for PointHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        match value.type_ as _ {
            SLVS_E_POINT_IN_2D => Ok(PointHandle::OnWorkplane(value.into())),
            SLVS_E_POINT_IN_3D => Ok(PointHandle::In3d(value.into())),
            _ => Err("Expected Slvs_Entity type of point"),
        }
    }
}

impl TryFrom<SomeEntityHandle> for PointHandle {
    type Error = &'static str;

    fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
        match value {
            SomeEntityHandle::PointOnWorkplane(h) => Ok(PointHandle::OnWorkplane(h)),
            SomeEntityHandle::PointIn3d(h) => Ok(PointHandle::In3d(h)),
            _ => Err("Expected Point variant of SomeEntityHandle"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity handle for some 2d things that can be a projection target
////////////////////////////////////////////////////////////////////////////////

pub trait As2dProjectionTarget: AsEntityData {}

#[enum_dispatch(ProjectionTargetHandle)]
trait AsProjectionTargetHandle {}
impl AsProjectionTargetHandle for EntityHandle<LineSegment<OnWorkplane>> {}
impl AsProjectionTargetHandle for EntityHandle<LineSegment<In3d>> {}
impl AsProjectionTargetHandle for EntityHandle<Normal> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
pub enum ProjectionTargetHandle {
    LineSegmentOnWorkplane(EntityHandle<LineSegment<OnWorkplane>>),
    LineSegmentIn3d(EntityHandle<LineSegment<In3d>>),
    Normal(EntityHandle<Normal>),
}

impl TryFrom<Slvs_Entity> for ProjectionTargetHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        match value.type_ as _ {
            SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => {
                Ok(ProjectionTargetHandle::Normal(value.into()))
            }
            SLVS_E_LINE_SEGMENT => match value.wrkpl {
                0 => Ok(ProjectionTargetHandle::LineSegmentIn3d(value.into())),
                _ => Ok(ProjectionTargetHandle::LineSegmentOnWorkplane(value.into())),
            },
            _ => Err("Expected Slvs_Entity type of line segment or normal"),
        }
    }
}

impl TryFrom<SomeEntityHandle> for ProjectionTargetHandle {
    type Error = &'static str;

    fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
        match value {
            SomeEntityHandle::LineSegmentOnWorkplane(h) => {
                Ok(ProjectionTargetHandle::LineSegmentOnWorkplane(h))
            }
            SomeEntityHandle::LineSegmentIn3d(h) => Ok(ProjectionTargetHandle::LineSegmentIn3d(h)),
            SomeEntityHandle::Normal(h) => Ok(ProjectionTargetHandle::Normal(h)),
            _ => Err("Expected LineSegment or Normal variant of SomeEntityHandle"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity Data
////////////////////////////////////////////////////////////////////////////////

pub trait AsEntityData: Copy + Debug {
    fn from_system(sys: &System, entity_handle: &EntityHandle<Self>) -> Result<Self, &'static str>;

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
}
