/*!
Traits that are implemented across SolveSpace's entities.

These are all sealed, and not intended to be implemented for types outside of `slvs`.
*/

use std::any::Any;

use crate::{bindings::Slvs_hGroup, System};

/// An object that has a handle.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsHandle: private::Sealed {
    fn handle(&self) -> u32;
}

/// An object that belongs to a group.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsGroup: private::Sealed {
    fn group(&self) -> Slvs_hGroup;
}

/// An object that has an associated const, for identification within the C library.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsSlvsType: private::Sealed {
    fn slvs_type(&self) -> i32;
}

/// Used for downcasting.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

/// Used to convert from the C library representation to the Rust library.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait FromSystem: private::Sealed {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized;
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_element {
    (
        $slvs_type:ident,
        $(#[$doc:meta])*
        struct $name:ident {
            $(
                $(#[$member_doc:meta])*
                $field_name:ident: $field_type:ty,
            )*
        }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
        $(#[$doc])*
        pub struct $name {
            #[doc = concat!("The group that `", stringify!($name), "` belongs to.")]
            pub group: Group,
            $(  $(#[$member_doc])*
                pub $field_name: $field_type,
            )*
        }

        impl $name {
            #[doc = concat!("Create a new `", stringify!($name), "` instance.")]
            pub fn new(group: Group, $($field_name: $field_type,)*) -> Self {
                Self{
                    group,
                    $($field_name,)*
                }
            }
        }

        impl AsGroup for $name {
            fn group(&self) -> Slvs_hGroup {
                self.group.handle()
            }
        }

        impl AsSlvsType for $name {
            fn slvs_type(&self) -> i32 {
                $slvs_type as _
            }
        }
    };
}

mod private {
    use crate::{constraint::*, entity::*, group::Group};

    pub trait Sealed {}

    // Constraints
    impl Sealed for Box<dyn AsConstraintHandle> {}
    impl<C: AsConstraintData> Sealed for ConstraintHandle<C> {}
    impl Sealed for Angle {}
    impl Sealed for ArcArcDifference {}
    impl Sealed for ArcArcLenRatio {}
    impl Sealed for ArcLineDifference {}
    impl Sealed for ArcLineLenRatio {}
    impl Sealed for ArcLineTangent {}
    impl Sealed for AtMidpoint {}
    impl Sealed for CubicLineTangent {}
    impl<CA, CB> Sealed for CurveCurveTangent<CA, CB>
    where
        CA: AsCurve,
        CB: AsCurve,
    {
    }
    impl<A: AsArc> Sealed for Diameter<A> {}
    impl Sealed for EqLenPtLineD {}
    impl Sealed for EqPtLnDistances {}
    impl Sealed for EqualAngle {}
    impl Sealed for EqualLengthLines {}
    impl Sealed for EqualLineArcLen {}
    impl<AA, AB> Sealed for EqualRadius<AA, AB>
    where
        AA: AsArc,
        AB: AsArc,
    {
    }
    impl Sealed for Horizontal {}
    impl Sealed for LengthDifference {}
    impl Sealed for LengthRatio {}
    impl Sealed for Parallel {}
    impl Sealed for Perpendicular {}
    impl Sealed for PointsCoincident {}
    impl<L: AsProjectionTarget> Sealed for ProjPtDistance<L> {}
    impl Sealed for PtInPlane {}
    impl Sealed for PtLineDistance {}
    impl<A: AsArc> Sealed for PtOnCircle<A> {}
    impl Sealed for PtOnLine {}
    impl Sealed for PtPlaneDistance {}
    impl Sealed for PtPtDistance {}
    impl Sealed for SameOrientation {}
    impl Sealed for Symmetric {}
    impl Sealed for SymmetricHoriz {}
    impl Sealed for SymmetricLine {}
    impl Sealed for SymmetricVert {}
    impl Sealed for Vertical {}
    impl Sealed for WhereDragged {}

    // Entities
    impl Sealed for Box<dyn AsEntityHandle> {}
    impl<E: AsEntityData> Sealed for EntityHandle<E> {}
    impl Sealed for ArcOfCircle {}
    impl Sealed for Circle {}
    impl Sealed for Cubic {}
    impl Sealed for Distance {}
    impl Sealed for LineSegment {}
    impl Sealed for Normal {}
    impl Sealed for Point {}
    impl Sealed for Workplane {}

    // Groups
    impl Sealed for Group {}
}
