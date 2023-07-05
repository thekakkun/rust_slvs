/*!
A constraint is a geometric property of an entity, or a relationship between
multiple entities.

The [`ConstraintHandle<C>`] stores information about the type of constraint in a phantom type.
This is used to figure out what kind of data it needs to return when querying the
system.

The constraint data is defined and added to the system using structs that implement
[`AsConstraintData`].
*/

pub use angle::Angle;
pub use arc_arc_difference::ArcArcDifference;
pub use arc_arc_len_ratio::ArcArcLenRatio;
pub use arc_line_difference::ArcLineDifference;
pub use arc_line_len_ratio::ArcLineLenRatio;
pub use arc_line_tangent::ArcLineTangent;
pub use at_midpoint::AtMidpoint;
pub use cubic_line_tangent::CubicLineTangent;
pub use curve_curve_tangent::CurveCurveTangent;
pub use diameter::Diameter;
pub use eq_len_pt_line_d::EqLenPtLineD;
pub use eq_pt_ln_distances::EqPtLnDistances;
pub use equal_angle::EqualAngle;
pub use equal_length_lines::EqualLengthLines;
pub use equal_line_arc_len::EqualLineArcLen;
pub use equal_radius::EqualRadius;
pub use horizontal::Horizontal;
pub use length_difference::LengthDifference;
pub use length_ratio::LengthRatio;
pub use parallel::Parallel;
pub use perpendicular::Perpendicular;
pub use points_coincident::PointsCoincident;
pub use proj_pt_distance::ProjPtDistance;
pub use pt_in_plane::PtInPlane;
pub use pt_line_distance::PtLineDistance;
pub use pt_on_circle::PtOnCircle;
pub use pt_on_line::PtOnLine;
pub use pt_plane_distance::PtPlaneDistance;
pub use pt_pt_distance::PtPtDistance;
pub use same_orientation::SameOrientation;
pub use symmetric::Symmetric;
pub use symmetric_horiz::SymmetricHoriz;
pub use symmetric_line::SymmetricLine;
pub use symmetric_vert::SymmetricVert;
pub use vertical::Vertical;
pub use where_dragged::WhereDragged;

mod angle;
mod arc_arc_difference;
mod arc_arc_len_ratio;
mod arc_line_difference;
mod arc_line_len_ratio;
mod arc_line_tangent;
mod at_midpoint;
mod cubic_line_tangent;
mod curve_curve_tangent;
mod diameter;
mod eq_len_pt_line_d;
mod eq_pt_ln_distances;
mod equal_angle;
mod equal_length_lines;
mod equal_line_arc_len;
mod equal_radius;
mod horizontal;
mod length_difference;
mod length_ratio;
mod parallel;
mod perpendicular;
mod points_coincident;
mod proj_pt_distance;
mod pt_in_plane;
mod pt_line_distance;
mod pt_on_circle;
mod pt_on_line;
mod pt_plane_distance;
mod pt_pt_distance;
mod same_orientation;
mod symmetric;
mod symmetric_horiz;
mod symmetric_line;
mod symmetric_vert;
mod vertical;
mod where_dragged;

use serde::{Deserialize, Serialize};
use std::{any::TypeId, fmt::Debug, marker::PhantomData};

use crate::{
    bindings::{
        Slvs_Constraint, Slvs_hEntity, SLVS_C_ANGLE, SLVS_C_ARC_ARC_DIFFERENCE,
        SLVS_C_ARC_ARC_LEN_RATIO, SLVS_C_ARC_LINE_DIFFERENCE, SLVS_C_ARC_LINE_LEN_RATIO,
        SLVS_C_ARC_LINE_TANGENT, SLVS_C_AT_MIDPOINT, SLVS_C_CUBIC_LINE_TANGENT, SLVS_C_EQUAL_ANGLE,
        SLVS_C_EQUAL_LENGTH_LINES, SLVS_C_EQUAL_LINE_ARC_LEN, SLVS_C_EQ_LEN_PT_LINE_D,
        SLVS_C_EQ_PT_LN_DISTANCES, SLVS_C_HORIZONTAL, SLVS_C_LENGTH_DIFFERENCE,
        SLVS_C_LENGTH_RATIO, SLVS_C_PARALLEL, SLVS_C_PERPENDICULAR, SLVS_C_POINTS_COINCIDENT,
        SLVS_C_PT_FACE_DISTANCE, SLVS_C_PT_IN_PLANE, SLVS_C_PT_LINE_DISTANCE, SLVS_C_PT_ON_FACE,
        SLVS_C_PT_ON_LINE, SLVS_C_PT_PLANE_DISTANCE, SLVS_C_PT_PT_DISTANCE,
        SLVS_C_SAME_ORIENTATION, SLVS_C_SYMMETRIC, SLVS_C_SYMMETRIC_HORIZ, SLVS_C_SYMMETRIC_LINE,
        SLVS_C_SYMMETRIC_VERT, SLVS_C_VERTICAL, SLVS_C_WHERE_DRAGGED,
    },
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
};

/// An object wrapping a handle for a constraint
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsConstraintHandle: AsHandle {}

/// Wrapper for a constraint handle.
///
/// The `phantom` member holds information about what type of constraint it references.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintHandle<C: AsConstraintData> {
    /// The constraint handle
    pub handle: u32,
    pub(super) phantom: PhantomData<C>,
}

impl<C: AsConstraintData> ConstraintHandle<C> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<C: AsConstraintData> AsConstraintHandle for ConstraintHandle<C> {}
impl<C: AsConstraintData> AsHandle for ConstraintHandle<C> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl<C: AsConstraintData + 'static> TryFrom<SomeConstraintHandle> for ConstraintHandle<C> {
    type Error = &'static str;

    fn try_from(value: SomeConstraintHandle) -> Result<Self, Self::Error> {
        match value {
            SomeConstraintHandle::Angle(h) if TypeId::of::<C>() == TypeId::of::<Angle>() => {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::ArcArcDifference(h)
                if TypeId::of::<C>() == TypeId::of::<ArcArcDifference>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::ArcArcLenRatio(h)
                if TypeId::of::<C>() == TypeId::of::<ArcArcLenRatio>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::ArcLineDifference(h)
                if TypeId::of::<C>() == TypeId::of::<ArcLineDifference>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::ArcLineLenRatio(h)
                if TypeId::of::<C>() == TypeId::of::<ArcLineLenRatio>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::ArcLineTangent(h)
                if TypeId::of::<C>() == TypeId::of::<ArcLineTangent>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::AtMidpoint(h)
                if TypeId::of::<C>() == TypeId::of::<AtMidpoint>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::CubicLineTangent(h)
                if TypeId::of::<C>() == TypeId::of::<CubicLineTangent>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::CurveCurveTangent(h)
                if TypeId::of::<C>() == TypeId::of::<CurveCurveTangent>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::Diameter(h) if TypeId::of::<C>() == TypeId::of::<Diameter>() => {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::EqLenPtLineD(h)
                if TypeId::of::<C>() == TypeId::of::<EqLenPtLineD>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::EqPtLnDistances(h)
                if TypeId::of::<C>() == TypeId::of::<EqPtLnDistances>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::EqualAngle(h)
                if TypeId::of::<C>() == TypeId::of::<EqualAngle>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::EqualLengthLines(h)
                if TypeId::of::<C>() == TypeId::of::<EqualLengthLines>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::EqualLineArcLen(h)
                if TypeId::of::<C>() == TypeId::of::<EqualLineArcLen>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::EqualRadius(h)
                if TypeId::of::<C>() == TypeId::of::<EqualRadius>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::Horizontal(h)
                if TypeId::of::<C>() == TypeId::of::<Horizontal>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::LengthDifference(h)
                if TypeId::of::<C>() == TypeId::of::<LengthDifference>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::LengthRatio(h)
                if TypeId::of::<C>() == TypeId::of::<LengthRatio>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::Parallel(h) if TypeId::of::<C>() == TypeId::of::<Parallel>() => {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::Perpendicular(h)
                if TypeId::of::<C>() == TypeId::of::<Perpendicular>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::PointsCoincident(h)
                if TypeId::of::<C>() == TypeId::of::<PointsCoincident>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::ProjPtDistance(h)
                if TypeId::of::<C>() == TypeId::of::<ProjPtDistance>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::PtInPlane(h)
                if TypeId::of::<C>() == TypeId::of::<PtInPlane>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::PtLineDistance(h)
                if TypeId::of::<C>() == TypeId::of::<PtLineDistance>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::PtOnCircle(h)
                if TypeId::of::<C>() == TypeId::of::<PtOnCircle>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::PtOnLine(h) if TypeId::of::<C>() == TypeId::of::<PtOnLine>() => {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::PtPlaneDistance(h)
                if TypeId::of::<C>() == TypeId::of::<PtPlaneDistance>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::PtPtDistance(h)
                if TypeId::of::<C>() == TypeId::of::<PtPtDistance>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::SameOrientation(h)
                if TypeId::of::<C>() == TypeId::of::<SameOrientation>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::Symmetric(h)
                if TypeId::of::<C>() == TypeId::of::<Symmetric>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::SymmetricHoriz(h)
                if TypeId::of::<C>() == TypeId::of::<SymmetricHoriz>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::SymmetricLine(h)
                if TypeId::of::<C>() == TypeId::of::<SymmetricLine>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::SymmetricVert(h)
                if TypeId::of::<C>() == TypeId::of::<SymmetricVert>() =>
            {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::Vertical(h) if TypeId::of::<C>() == TypeId::of::<Vertical>() => {
                Ok(Self::new(h))
            }
            SomeConstraintHandle::WhereDragged(h)
                if TypeId::of::<C>() == TypeId::of::<WhereDragged>() =>
            {
                Ok(Self::new(h))
            }
            _ => Err("Variant must match target handle type."),
        }
    }
}

impl<C: AsConstraintData> From<Slvs_Constraint> for ConstraintHandle<C> {
    fn from(value: Slvs_Constraint) -> Self {
        ConstraintHandle::new(value.h)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "handle")]
pub enum SomeConstraintHandle {
    Angle(u32),
    ArcArcDifference(u32),
    ArcArcLenRatio(u32),
    ArcLineDifference(u32),
    ArcLineLenRatio(u32),
    ArcLineTangent(u32),
    AtMidpoint(u32),
    CubicLineTangent(u32),
    CurveCurveTangent(u32),
    Diameter(u32),
    EqLenPtLineD(u32),
    EqPtLnDistances(u32),
    EqualAngle(u32),
    EqualLengthLines(u32),
    EqualLineArcLen(u32),
    EqualRadius(u32),
    Horizontal(u32),
    LengthDifference(u32),
    LengthRatio(u32),
    Parallel(u32),
    Perpendicular(u32),
    PointsCoincident(u32),
    ProjPtDistance(u32),
    PtInPlane(u32),
    PtLineDistance(u32),
    PtOnCircle(u32),
    PtOnLine(u32),
    PtPlaneDistance(u32),
    PtPtDistance(u32),
    SameOrientation(u32),
    Symmetric(u32),
    SymmetricHoriz(u32),
    SymmetricLine(u32),
    SymmetricVert(u32),
    Vertical(u32),
    WhereDragged(u32),
}

impl AsConstraintHandle for SomeConstraintHandle {}
impl AsHandle for SomeConstraintHandle {
    fn handle(&self) -> u32 {
        match self {
            SomeConstraintHandle::Angle(h)
            | SomeConstraintHandle::ArcArcDifference(h)
            | SomeConstraintHandle::ArcArcLenRatio(h)
            | SomeConstraintHandle::ArcLineDifference(h)
            | SomeConstraintHandle::ArcLineLenRatio(h)
            | SomeConstraintHandle::ArcLineTangent(h)
            | SomeConstraintHandle::AtMidpoint(h)
            | SomeConstraintHandle::CubicLineTangent(h)
            | SomeConstraintHandle::CurveCurveTangent(h)
            | SomeConstraintHandle::Diameter(h)
            | SomeConstraintHandle::EqLenPtLineD(h)
            | SomeConstraintHandle::EqPtLnDistances(h)
            | SomeConstraintHandle::EqualAngle(h)
            | SomeConstraintHandle::EqualLengthLines(h)
            | SomeConstraintHandle::EqualLineArcLen(h)
            | SomeConstraintHandle::EqualRadius(h)
            | SomeConstraintHandle::Horizontal(h)
            | SomeConstraintHandle::LengthDifference(h)
            | SomeConstraintHandle::LengthRatio(h)
            | SomeConstraintHandle::Parallel(h)
            | SomeConstraintHandle::Perpendicular(h)
            | SomeConstraintHandle::PointsCoincident(h)
            | SomeConstraintHandle::ProjPtDistance(h)
            | SomeConstraintHandle::PtInPlane(h)
            | SomeConstraintHandle::PtLineDistance(h)
            | SomeConstraintHandle::PtOnCircle(h)
            | SomeConstraintHandle::PtOnLine(h)
            | SomeConstraintHandle::PtPlaneDistance(h)
            | SomeConstraintHandle::PtPtDistance(h)
            | SomeConstraintHandle::SameOrientation(h)
            | SomeConstraintHandle::Symmetric(h)
            | SomeConstraintHandle::SymmetricHoriz(h)
            | SomeConstraintHandle::SymmetricLine(h)
            | SomeConstraintHandle::SymmetricVert(h)
            | SomeConstraintHandle::Vertical(h)
            | SomeConstraintHandle::WhereDragged(h) => *h,
        }
    }
}

impl<C: AsConstraintData> From<ConstraintHandle<C>> for SomeConstraintHandle {
    fn from(value: ConstraintHandle<C>) -> Self {
        C::to_some_handle(value.handle())
    }
}

impl From<Slvs_Constraint> for SomeConstraintHandle {
    fn from(value: Slvs_Constraint) -> Self {
        match value.type_ as _ {
            SLVS_C_POINTS_COINCIDENT => SomeConstraintHandle::PointsCoincident(value.h),
            SLVS_C_PT_PT_DISTANCE => SomeConstraintHandle::PtPtDistance(value.h),
            SLVS_C_PT_PLANE_DISTANCE => SomeConstraintHandle::PtPlaneDistance(value.h),
            SLVS_C_PT_LINE_DISTANCE => SomeConstraintHandle::PtLineDistance(value.h),
            SLVS_C_PT_IN_PLANE => SomeConstraintHandle::PtInPlane(value.h),
            SLVS_C_PT_ON_LINE => SomeConstraintHandle::PtOnLine(value.h),
            SLVS_C_EQUAL_LENGTH_LINES => SomeConstraintHandle::EqualLengthLines(value.h),
            SLVS_C_LENGTH_RATIO => SomeConstraintHandle::LengthRatio(value.h),
            SLVS_C_EQ_LEN_PT_LINE_D => SomeConstraintHandle::EqLenPtLineD(value.h),
            SLVS_C_EQ_PT_LN_DISTANCES => SomeConstraintHandle::EqPtLnDistances(value.h),
            SLVS_C_EQUAL_ANGLE => SomeConstraintHandle::EqualAngle(value.h),
            SLVS_C_EQUAL_LINE_ARC_LEN => SomeConstraintHandle::EqualLineArcLen(value.h),
            SLVS_C_SYMMETRIC => SomeConstraintHandle::Symmetric(value.h),
            SLVS_C_SYMMETRIC_HORIZ => SomeConstraintHandle::SymmetricHoriz(value.h),
            SLVS_C_SYMMETRIC_VERT => SomeConstraintHandle::SymmetricVert(value.h),
            SLVS_C_SYMMETRIC_LINE => SomeConstraintHandle::SymmetricLine(value.h),
            SLVS_C_AT_MIDPOINT => SomeConstraintHandle::AtMidpoint(value.h),
            SLVS_C_HORIZONTAL => SomeConstraintHandle::Horizontal(value.h),
            SLVS_C_VERTICAL => SomeConstraintHandle::Vertical(value.h),
            SLVS_C_SAME_ORIENTATION => SomeConstraintHandle::SameOrientation(value.h),
            SLVS_C_ANGLE => SomeConstraintHandle::Angle(value.h),
            SLVS_C_PARALLEL => SomeConstraintHandle::Parallel(value.h),
            SLVS_C_PERPENDICULAR => SomeConstraintHandle::Perpendicular(value.h),
            SLVS_C_ARC_LINE_TANGENT => SomeConstraintHandle::ArcLineTangent(value.h),
            SLVS_C_CUBIC_LINE_TANGENT => SomeConstraintHandle::CubicLineTangent(value.h),
            SLVS_C_WHERE_DRAGGED => SomeConstraintHandle::WhereDragged(value.h),
            SLVS_C_LENGTH_DIFFERENCE => SomeConstraintHandle::LengthDifference(value.h),
            SLVS_C_ARC_ARC_LEN_RATIO => SomeConstraintHandle::ArcArcLenRatio(value.h),
            SLVS_C_ARC_LINE_LEN_RATIO => SomeConstraintHandle::ArcLineLenRatio(value.h),
            SLVS_C_ARC_ARC_DIFFERENCE => SomeConstraintHandle::ArcArcDifference(value.h),
            SLVS_C_ARC_LINE_DIFFERENCE => SomeConstraintHandle::ArcLineDifference(value.h),
            SLVS_C_PT_FACE_DISTANCE | SLVS_C_PT_ON_FACE => {
                panic!("Face entity not defined for library.")
            }
            _ => panic!("Unknown Slvs_Constraint type value {}", value.type_),
        }
    }
}

/// An object that holds information about a constraint.
///
/// This trait is sealed and cannot be implemented for types outside of `slvs`.
pub trait AsConstraintData: private::Sealed + AsGroup + AsSlvsType + FromSystem {
    fn to_some_handle(handle: u32) -> SomeConstraintHandle;

    fn workplane(&self) -> Option<Slvs_hEntity>;

    fn val(&self) -> Option<f64> {
        None
    }
    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        None
    }
    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        None
    }
    fn others(&self) -> [bool; 2] {
        [false, false]
    }
}

mod private {
    use super::AsConstraintData;

    pub trait Sealed {}
    impl<C: AsConstraintData> Sealed for C {}
}
