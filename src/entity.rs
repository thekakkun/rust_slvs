/*!
An entity is a geometric thing, like a point or a line segment or a circle.

The [`EntityHandle<E>`] stores information about the type of entity in a phantom type, which
is used to ensure that handles reference the correct type of entity when defining other
entities and constraints.

They are defined and added to the using structs that implement [`AsEntityData`],
and can be retrieved with the [`EntityHandle`] struct, which is a wrapper for the
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

use serde::{Deserialize, Serialize};
use std::{
    any::{type_name, Any},
    fmt::Debug,
    marker::PhantomData,
};

use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE, SLVS_E_CUBIC,
        SLVS_E_DISTANCE, SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D,
        SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D, SLVS_E_WORKPLANE,
    },
    element::{AsAny, AsGroup, AsHandle, AsSlvsType, FromSystem},
};

/// An object wrapping a handle for an entity.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsEntityHandle: AsAny + AsHandle {
    /// Get the type name as a string.
    fn type_name(&self) -> &'static str;
}

impl AsAny for Box<dyn AsEntityHandle> {
    fn as_any(&self) -> &dyn Any {
        self.as_ref().as_any()
    }
}

impl AsHandle for Box<dyn AsEntityHandle> {
    fn handle(&self) -> u32 {
        self.as_ref().handle()
    }
}

impl AsEntityHandle for Box<dyn AsEntityHandle> {
    fn type_name(&self) -> &'static str {
        self.as_ref().type_name()
    }
}

impl Debug for Box<dyn AsEntityHandle> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityHandle")
            .field("handle", &self.handle())
            .field("type", &self.type_name())
            .finish()
    }
}

impl From<Slvs_Entity> for Box<dyn AsEntityHandle> {
    fn from(value: Slvs_Entity) -> Self {
        match value.type_ as _ {
            SLVS_E_ARC_OF_CIRCLE => {
                Box::new(EntityHandle::<ArcOfCircle>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            SLVS_E_CIRCLE => {
                Box::new(EntityHandle::<Circle>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            SLVS_E_CUBIC => {
                Box::new(EntityHandle::<Cubic>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            SLVS_E_DISTANCE => {
                Box::new(EntityHandle::<Distance>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            SLVS_E_LINE_SEGMENT => {
                Box::new(EntityHandle::<LineSegment>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => {
                Box::new(EntityHandle::<Normal>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            SLVS_E_POINT_IN_2D | SLVS_E_POINT_IN_3D => {
                Box::new(EntityHandle::<Point>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            SLVS_E_WORKPLANE => {
                Box::new(EntityHandle::<Workplane>::new(value.h)) as Box<dyn AsEntityHandle>
            }
            _ => panic!("Unknown Slvs_Entity type value {}", value.type_),
        }
    }
}

/// Wrapper for an entity handle.
///
/// The `phantom` member holds information about what type of entity it references,
/// which is used to check that entity definitions receive the correct type of entity
/// handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityHandle<E: AsEntityData> {
    /// The entity handle
    pub handle: u32,
    pub(super) phantom: PhantomData<E>,
}

impl<E: AsEntityData> EntityHandle<E> {
    pub(crate) fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<E: AsEntityData + 'static> AsAny for EntityHandle<E> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<E: AsEntityData> AsHandle for EntityHandle<E> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl<E: AsEntityData + 'static> AsEntityHandle for EntityHandle<E> {
    fn type_name(&self) -> &'static str {
        type_name::<E>()
    }
}

impl<E: AsEntityData + Copy + 'static> TryFrom<&Box<dyn AsEntityHandle>> for EntityHandle<E> {
    type Error = &'static str;

    fn try_from(value: &Box<dyn AsEntityHandle>) -> Result<Self, Self::Error> {
        if let Some(entity_handle) = value.as_any().downcast_ref::<EntityHandle<E>>() {
            Ok(*entity_handle)
        } else {
            Err("Can only downcast boxed value into same type")
        }
    }
}

impl<E: AsEntityData> From<Slvs_Entity> for EntityHandle<E> {
    fn from(value: Slvs_Entity) -> Self {
        EntityHandle::new(value.h)
    }
}

/// An entity that has a radius.
///
/// Used as arguments when creating the [Diameter][crate::constraint::Diameter],
/// [EqualRadius][crate::constraint::EqualRadius], and [PtOnCircle][crate::constraint::PtOnCircle]
/// constraints.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsArc: AsEntityData {}
impl AsArc for ArcOfCircle {}
impl AsArc for Circle {}

/// An entity that is a curve with start and end points.
///
/// Used as arguments when creating the [CurveCurveTangent][crate::constraint::CurveCurveTangent]
/// constraint.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsCurve: AsEntityData {}
impl AsCurve for ArcOfCircle {}
impl AsCurve for Cubic {}

/// An entity that is a 2d projection target.
///
/// Used as an argument when creating the [ProjPtDistance][crate::constraint::ProjPtDistance]
/// constraint.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsProjectionTarget: AsEntityData {}
impl AsProjectionTarget for LineSegment {}
impl AsProjectionTarget for Normal {}

/// An object that holds information about an entity.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsEntityData: private::Sealed + AsGroup + AsSlvsType + FromSystem {
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
