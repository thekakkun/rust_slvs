/*!
An entity is a geometric thing, like a point or a line segment or a circle.

Entities are sketched [OnWorkplane][crate::target::OnWorkplane] or [OnWorkplane][crate::target::In3d].
The [EntityHandle], stores information about the type of entity and where it was sketched,
which are used to ensure that other entities and constraints receive a handle for the expected type of entity.

They are defined and added to the using structs that implement [AsEntityData],
and can be retrieved with the [EntityHandle] struct, which is a wrapper for the
entity handle.
*/

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

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::{
    any::{type_name, TypeId},
    fmt::Debug,
    marker::PhantomData,
};

use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE, SLVS_E_CUBIC,
        SLVS_E_DISTANCE, SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D,
        SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D, SLVS_E_WORKPLANE,
    },
    element::{AsGroup, AsHandle, AsSlvsType},
    System,
};

pub(crate) trait AsEntityHandle: AsHandle {}

/// Wrapper for an entity handle.
///
/// The type argument holds information about what type of entity it references,
/// which is used to check that entity definitions receive the correct type of entity handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityHandle<T: AsEntityData> {
    pub handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<E: AsEntityData> EntityHandle<E> {
    pub(crate) fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<E: AsEntityData> AsEntityHandle for EntityHandle<E> {}
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

pub trait AsRadiused: AsEntityData {}
impl AsRadiused for ArcOfCircle {}
impl AsRadiused for Circle {}

pub trait AsCurve: AsEntityData {}
impl AsCurve for ArcOfCircle {}
impl AsCurve for Cubic {}

pub trait AsProjectionTarget: AsEntityData {}
impl AsProjectionTarget for LineSegment {}
impl AsProjectionTarget for Normal {}

/// An object that holds information about an entity.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsEntityData: private::Sealed + AsGroup + AsSlvsType {
    #[doc(hidden)]
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }
    #[doc(hidden)]
    fn points(&self) -> Option<[Slvs_hEntity; 4]> {
        None
    }
    #[doc(hidden)]
    fn normal(&self) -> Option<Slvs_hEntity> {
        None
    }
    #[doc(hidden)]
    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }
    #[doc(hidden)]
    fn param_vals(&self) -> [Option<f64>; 4] {
        [None, None, None, None]
    }
}

mod private {
    use super::AsEntityData;

    pub trait Sealed {}
    impl<E: AsEntityData> Sealed for E {}
}

// /// A thing that wraps a handle for an entity.
// ///
// /// This trait is sealed and cannot be implemented for types outside of `slvs`.
// #[enum_dispatch(SomeEntityHandle)]
// pub trait AsEntityHandle: AsHandle {}
// impl<E: AsEntityData> AsEntityHandle for EntityHandle<E> {}

// /// Wrapper enum for the possible types of [`EntityHandle`].
// #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
// #[enum_dispatch]
// pub enum SomeEntityHandle {
//     ArcOfCircle(EntityHandle<ArcOfCircle>),
//     Circle(EntityHandle<Circle>),
//     Cubic(EntityHandle<Cubic>),
//     Distance(EntityHandle<Distance>),
//     LineSegment(EntityHandle<LineSegment>),
//     Normal(EntityHandle<Normal>),
//     Point(EntityHandle<Point>),
//     Workplane(EntityHandle<Workplane>),
// }

// impl From<Slvs_Entity> for SomeEntityHandle {
//     fn from(value: Slvs_Entity) -> Self {
//         match value.type_ as _ {
//             SLVS_E_ARC_OF_CIRCLE => Self::ArcOfCircle(value.into()),
//             SLVS_E_CIRCLE => match value.wrkpl {
//                 0 => Self::CircleIn3d(value.try_into().unwrap()),
//                 _ => Self::CircleOnWorkplane(value.try_into().unwrap()),
//             },
//             SLVS_E_CUBIC => match value.wrkpl {
//                 0 => Self::CubicIn3d(value.try_into().unwrap()),
//                 _ => Self::CubicOnWorkplane(value.try_into().unwrap()),
//             },
//             SLVS_E_DISTANCE => match value.wrkpl {
//                 0 => Self::DistanceIn3d(value.try_into().unwrap()),
//                 _ => Self::DistanceOnWorkplane(value.try_into().unwrap()),
//             },
//             SLVS_E_LINE_SEGMENT => match value.wrkpl {
//                 0 => Self::LineSegmentIn3d(value.try_into().unwrap()),
//                 _ => Self::LineSegmentOnWorkplane(value.try_into().unwrap()),
//             },
//             SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => Self::Normal(value.into()),
//             SLVS_E_POINT_IN_2D => Self::PointOnWorkplane(value.try_into().unwrap()),
//             SLVS_E_POINT_IN_3D => Self::PointIn3d(value.try_into().unwrap()),
//             SLVS_E_WORKPLANE => Self::Workplane(value.into()),
//             _ => panic!("Unknown Slvs_Entity type value {}", value.type_),
//         }
//     }
// }

// impl From<ArcHandle> for SomeEntityHandle {
//     fn from(value: ArcHandle) -> Self {
//         match value {
//             ArcHandle::ArcOfCircle(h) => Self::ArcOfCircle(h),
//             ArcHandle::CircleOnWorkplane(h) => Self::CircleOnWorkplane(h),
//             ArcHandle::CircleIn3d(h) => Self::CircleIn3d(h),
//         }
//     }
// }

// impl From<CurveHandle> for SomeEntityHandle {
//     fn from(value: CurveHandle) -> Self {
//         match value {
//             CurveHandle::ArcOfCircle(h) => Self::ArcOfCircle(h),
//             CurveHandle::CubicOnWorkplane(h) => Self::CubicOnWorkplane(h),
//             CurveHandle::CubicIn3d(h) => Self::CubicIn3d(h),
//         }
//     }
// }

// impl From<ProjectionTargetHandle> for SomeEntityHandle {
//     fn from(value: ProjectionTargetHandle) -> Self {
//         match value {
//             ProjectionTargetHandle::LineSegmentOnWorkplane(h) => Self::LineSegmentOnWorkplane(h),
//             ProjectionTargetHandle::LineSegmentIn3d(h) => Self::LineSegmentIn3d(h),
//             ProjectionTargetHandle::Normal(h) => Self::Normal(h),
//         }
//     }
// }

// #[enum_dispatch(ArcHandle)]
// trait AsArcHandle: AsEntityHandle {}
// impl AsArcHandle for EntityHandle<ArcOfCircle> {}
// impl AsArcHandle for EntityHandle<Circle> {}

// /// Wraps handles for some sort of arc entity.
// ///
// /// Used when defining
// /// [Diameter][crate::constraint::Diameter],
// /// [EqualRadius][crate::constraint::EqualRadius], and
// /// [PtOnCircle][crate::constraint::PtOnCircle] constraints.
// #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
// #[enum_dispatch]
// pub enum ArcHandle {
//     ArcOfCircle(EntityHandle<ArcOfCircle>),
//     Circle(EntityHandle<Circle>),
// }

// impl AsEntityHandle for ArcHandle {}

// impl TryFrom<Slvs_Entity> for ArcHandle {
//     type Error = &'static str;

//     fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
//         match value.type_ as _ {
//             SLVS_E_ARC_OF_CIRCLE => Ok(ArcHandle::ArcOfCircle(value.into())),
//             SLVS_E_CIRCLE => match value.wrkpl {
//                 0 => Ok(ArcHandle::CircleIn3d(value.into())),
//                 _ => Ok(ArcHandle::CircleOnWorkplane(value.into())),
//             },
//             _ => Err("Expected Slvs_Entity type of arc or circle"),
//         }
//     }
// }

// impl TryFrom<SomeEntityHandle> for ArcHandle {
//     type Error = &'static str;

//     fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
//         match value {
//             SomeEntityHandle::ArcOfCircle(h) => Ok(ArcHandle::ArcOfCircle(h)),
//             SomeEntityHandle::CircleOnWorkplane(h) => Ok(ArcHandle::CircleOnWorkplane(h)),
//             SomeEntityHandle::CircleIn3d(h) => Ok(ArcHandle::CircleIn3d(h)),
//             _ => Err("Expected Arc or Circle variant of SomeEntityHandle"),
//         }
//     }
// }

// #[enum_dispatch(CurveHandle)]
// trait AsCurveHandle: AsEntityHandle {}
// impl AsCurveHandle for EntityHandle<ArcOfCircle> {}
// impl AsCurveHandle for EntityHandle<Cubic> {}

// /// Wraps handles for some sort of curve entity.
// ///
// /// Used when defining
// /// [CurveCurveTangent][crate::constraint::CurveCurveTangent].
// #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
// #[enum_dispatch]
// pub enum CurveHandle {
//     ArcOfCircle(EntityHandle<ArcOfCircle>),
//     Cubic(EntityHandle<Cubic>),
// }

// impl AsEntityHandle for CurveHandle {}

// impl TryFrom<Slvs_Entity> for CurveHandle {
//     type Error = &'static str;

//     fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
//         match value.type_ as _ {
//             SLVS_E_ARC_OF_CIRCLE => Ok(CurveHandle::ArcOfCircle(value.into())),
//             SLVS_E_CUBIC => match value.wrkpl {
//                 0 => Ok(CurveHandle::CubicIn3d(value.into())),
//                 _ => Ok(CurveHandle::CubicOnWorkplane(value.into())),
//             },
//             _ => Err("Expected Slvs_Entity type of arc or cubic"),
//         }
//     }
// }

// impl TryFrom<SomeEntityHandle> for CurveHandle {
//     type Error = &'static str;

//     fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
//         match value {
//             SomeEntityHandle::ArcOfCircle(h) => Ok(CurveHandle::ArcOfCircle(h)),
//             SomeEntityHandle::CubicOnWorkplane(h) => Ok(CurveHandle::CubicOnWorkplane(h)),
//             SomeEntityHandle::CubicIn3d(h) => Ok(CurveHandle::CubicIn3d(h)),
//             _ => Err("Expected Arc or Cubic variant of SomeEntityHandle"),
//         }
//     }
// }

// #[enum_dispatch(ProjectionTargetHandle)]
// trait AsProjectionTargetHandle: AsEntityHandle {}
// impl AsProjectionTargetHandle for EntityHandle<LineSegment> {}
// impl AsProjectionTargetHandle for EntityHandle<Normal> {}

// /// Handles that can be passed when defining [ProjPtDistance][crate::constraint::ProjPtDistance].
// #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
// #[enum_dispatch]
// pub enum ProjectionTargetHandle {
//     LineSegment(EntityHandle<LineSegment>),
//     Normal(EntityHandle<Normal>),
// }

// impl AsEntityHandle for ProjectionTargetHandle {}

// impl TryFrom<Slvs_Entity> for ProjectionTargetHandle {
//     type Error = &'static str;

//     fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
//         match value.type_ as _ {
//             SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => {
//                 Ok(ProjectionTargetHandle::Normal(value.into()))
//             }
//             SLVS_E_LINE_SEGMENT => match value.wrkpl {
//                 0 => Ok(ProjectionTargetHandle::LineSegmentIn3d(value.into())),
//                 _ => Ok(ProjectionTargetHandle::LineSegmentOnWorkplane(value.into())),
//             },
//             _ => Err("Expected Slvs_Entity type of line segment or normal"),
//         }
//     }
// }

// impl TryFrom<SomeEntityHandle> for ProjectionTargetHandle {
//     type Error = &'static str;

//     fn try_from(value: SomeEntityHandle) -> Result<Self, Self::Error> {
//         match value {
//             SomeEntityHandle::LineSegmentOnWorkplane(h) => {
//                 Ok(ProjectionTargetHandle::LineSegmentOnWorkplane(h))
//             }
//             SomeEntityHandle::LineSegmentIn3d(h) => Ok(ProjectionTargetHandle::LineSegmentIn3d(h)),
//             SomeEntityHandle::Normal(h) => Ok(ProjectionTargetHandle::Normal(h)),
//             _ => Err("Expected LineSegment or Normal variant of SomeEntityHandle"),
//         }
//     }
// }
