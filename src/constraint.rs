/*!
A constraint is a geometric property of an entity, or a relationship between
multiple entities.

Add constraints to the [`crate::System`] by passing structs that implement
[`AsConstraintData`] to [`crate::System::constrain()`].
The handle struct [`ConstraintHandle`] is returned which can then be used retrieve
or modify constraint data.
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
pub use pt_face_distance::PtFaceDistance;
pub use pt_in_plane::PtInPlane;
pub use pt_line_distance::PtLineDistance;
pub use pt_on_circle::PtOnCircle;
pub use pt_on_face::PtOnFace;
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
mod pt_face_distance;
mod pt_in_plane;
mod pt_line_distance;
mod pt_on_circle;
mod pt_on_face;
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

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::{any::type_name, fmt::Debug, marker::PhantomData};

use crate::{
    bindings::{
        Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_ANGLE, SLVS_C_ARC_ARC_DIFFERENCE,
        SLVS_C_ARC_ARC_LEN_RATIO, SLVS_C_ARC_LINE_DIFFERENCE, SLVS_C_ARC_LINE_LEN_RATIO,
        SLVS_C_ARC_LINE_TANGENT, SLVS_C_AT_MIDPOINT, SLVS_C_CUBIC_LINE_TANGENT,
        SLVS_C_CURVE_CURVE_TANGENT, SLVS_C_DIAMETER, SLVS_C_EQUAL_ANGLE, SLVS_C_EQUAL_LENGTH_LINES,
        SLVS_C_EQUAL_LINE_ARC_LEN, SLVS_C_EQUAL_RADIUS, SLVS_C_EQ_LEN_PT_LINE_D,
        SLVS_C_EQ_PT_LN_DISTANCES, SLVS_C_HORIZONTAL, SLVS_C_LENGTH_DIFFERENCE,
        SLVS_C_LENGTH_RATIO, SLVS_C_PARALLEL, SLVS_C_PERPENDICULAR, SLVS_C_POINTS_COINCIDENT,
        SLVS_C_PROJ_PT_DISTANCE, SLVS_C_PT_FACE_DISTANCE, SLVS_C_PT_IN_PLANE,
        SLVS_C_PT_LINE_DISTANCE, SLVS_C_PT_ON_CIRCLE, SLVS_C_PT_ON_FACE, SLVS_C_PT_ON_LINE,
        SLVS_C_PT_PLANE_DISTANCE, SLVS_C_PT_PT_DISTANCE, SLVS_C_SAME_ORIENTATION, SLVS_C_SYMMETRIC,
        SLVS_C_SYMMETRIC_HORIZ, SLVS_C_SYMMETRIC_LINE, SLVS_C_SYMMETRIC_VERT, SLVS_C_VERTICAL,
        SLVS_C_WHERE_DRAGGED,
    },
    element::AsHandle,
    System,
};

////////////////////////////////////////////////////////////////////////////////
// Constraint Handle
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintHandle<C: AsConstraintData> {
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

impl<C: AsConstraintData> AsHandle for ConstraintHandle<C> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl<C: AsConstraintData> Debug for ConstraintHandle<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Constraint: {{h: {}, type: {}}}",
            self.handle,
            type_name::<C>()
        )
    }
}

impl<C: AsConstraintData> From<Slvs_Constraint> for ConstraintHandle<C> {
    fn from(value: Slvs_Constraint) -> Self {
        ConstraintHandle::new(value.h)
    }
}

#[enum_dispatch(SomeConstraintHandle)]
pub trait AsConstraintHandle: AsHandle {}
impl<C: AsConstraintData> AsConstraintHandle for ConstraintHandle<C> {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[enum_dispatch]
pub enum SomeConstraintHandle {
    Angle(ConstraintHandle<Angle>),
    ArcArcDifference(ConstraintHandle<ArcArcDifference>),
    ArcArcLenRatio(ConstraintHandle<ArcArcLenRatio>),
    ArcLineDifference(ConstraintHandle<ArcLineDifference>),
    ArcLineLenRatio(ConstraintHandle<ArcLineLenRatio>),
    ArcLineTangent(ConstraintHandle<ArcLineTangent>),
    AtMidpoint(ConstraintHandle<AtMidpoint>),
    CubicLineTangent(ConstraintHandle<CubicLineTangent>),
    CurveCurveTangent(ConstraintHandle<CurveCurveTangent>),
    Diameter(ConstraintHandle<Diameter>),
    EqLenPtLineD(ConstraintHandle<EqLenPtLineD>),
    EqPtLnDistances(ConstraintHandle<EqPtLnDistances>),
    EqualAngle(ConstraintHandle<EqualAngle>),
    EqualLengthLines(ConstraintHandle<EqualLengthLines>),
    EqualLineArcLen(ConstraintHandle<EqualLineArcLen>),
    EqualRadius(ConstraintHandle<EqualRadius>),
    Horizontal(ConstraintHandle<Horizontal>),
    LengthDifference(ConstraintHandle<LengthDifference>),
    LengthRatio(ConstraintHandle<LengthRatio>),
    Parallel(ConstraintHandle<Parallel>),
    Perpendicular(ConstraintHandle<Perpendicular>),
    PointsCoincident(ConstraintHandle<PointsCoincident>),
    ProjPtDistance(ConstraintHandle<ProjPtDistance>),
    PtFaceDistance(ConstraintHandle<PtFaceDistance>),
    PtInPlane(ConstraintHandle<PtInPlane>),
    PtLineDistance(ConstraintHandle<PtLineDistance>),
    PtOnCircle(ConstraintHandle<PtOnCircle>),
    PtOnFace(ConstraintHandle<PtOnFace>),
    PtOnLine(ConstraintHandle<PtOnLine>),
    PtPlaneDistance(ConstraintHandle<PtPlaneDistance>),
    PtPtDistance(ConstraintHandle<PtPtDistance>),
    SameOrientation(ConstraintHandle<SameOrientation>),
    Symmetric(ConstraintHandle<Symmetric>),
    SymmetricHoriz(ConstraintHandle<SymmetricHoriz>),
    SymmetricLine(ConstraintHandle<SymmetricLine>),
    SymmetricVert(ConstraintHandle<SymmetricVert>),
    Vertical(ConstraintHandle<Vertical>),
    WhereDragged(ConstraintHandle<WhereDragged>),
}

impl From<Slvs_Constraint> for SomeConstraintHandle {
    fn from(value: Slvs_Constraint) -> Self {
        match value.type_ as _ {
            SLVS_C_POINTS_COINCIDENT => Self::PointsCoincident(value.into()),
            SLVS_C_PT_PT_DISTANCE => Self::PtPtDistance(value.into()),
            SLVS_C_PT_PLANE_DISTANCE => Self::PtPlaneDistance(value.into()),
            SLVS_C_PT_LINE_DISTANCE => Self::PtLineDistance(value.into()),
            SLVS_C_PT_FACE_DISTANCE => Self::PtFaceDistance(value.into()),
            SLVS_C_PT_IN_PLANE => Self::PtInPlane(value.into()),
            SLVS_C_PT_ON_LINE => Self::PtOnLine(value.into()),
            SLVS_C_PT_ON_FACE => Self::PtOnFace(value.into()),
            SLVS_C_EQUAL_LENGTH_LINES => Self::EqualLengthLines(value.into()),
            SLVS_C_LENGTH_RATIO => Self::LengthRatio(value.into()),
            SLVS_C_EQ_LEN_PT_LINE_D => Self::EqLenPtLineD(value.into()),
            SLVS_C_EQ_PT_LN_DISTANCES => Self::EqPtLnDistances(value.into()),
            SLVS_C_EQUAL_ANGLE => Self::EqualAngle(value.into()),
            SLVS_C_EQUAL_LINE_ARC_LEN => Self::EqualLineArcLen(value.into()),
            SLVS_C_SYMMETRIC => Self::Symmetric(value.into()),
            SLVS_C_SYMMETRIC_HORIZ => Self::SymmetricHoriz(value.into()),
            SLVS_C_SYMMETRIC_VERT => Self::SymmetricVert(value.into()),
            SLVS_C_SYMMETRIC_LINE => Self::SymmetricLine(value.into()),
            SLVS_C_AT_MIDPOINT => Self::AtMidpoint(value.into()),
            SLVS_C_HORIZONTAL => Self::Horizontal(value.into()),
            SLVS_C_VERTICAL => Self::Vertical(value.into()),
            SLVS_C_DIAMETER => Self::Diameter(value.into()),
            SLVS_C_PT_ON_CIRCLE => Self::PtOnCircle(value.into()),
            SLVS_C_SAME_ORIENTATION => Self::SameOrientation(value.into()),
            SLVS_C_ANGLE => Self::Angle(value.into()),
            SLVS_C_PARALLEL => Self::Parallel(value.into()),
            SLVS_C_PERPENDICULAR => Self::Perpendicular(value.into()),
            SLVS_C_ARC_LINE_TANGENT => Self::ArcLineTangent(value.into()),
            SLVS_C_CUBIC_LINE_TANGENT => Self::CubicLineTangent(value.into()),
            SLVS_C_EQUAL_RADIUS => Self::EqualRadius(value.into()),
            SLVS_C_PROJ_PT_DISTANCE => Self::ProjPtDistance(value.into()),
            SLVS_C_WHERE_DRAGGED => Self::WhereDragged(value.into()),
            SLVS_C_CURVE_CURVE_TANGENT => Self::CurveCurveTangent(value.into()),
            SLVS_C_LENGTH_DIFFERENCE => Self::LengthDifference(value.into()),
            SLVS_C_ARC_ARC_LEN_RATIO => Self::ArcArcLenRatio(value.into()),
            SLVS_C_ARC_LINE_LEN_RATIO => Self::ArcLineLenRatio(value.into()),
            SLVS_C_ARC_ARC_DIFFERENCE => Self::ArcArcDifference(value.into()),
            SLVS_C_ARC_LINE_DIFFERENCE => Self::ArcLineDifference(value.into()),
            _ => panic!("Unknown Slvs_Constraint type value {}", value.type_),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity Data
////////////////////////////////////////////////////////////////////////////////

pub trait AsConstraintData: private::Sealed + Copy + Debug {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str>;

    fn slvs_type(&self) -> i32;
    fn workplane(&self) -> Option<Slvs_hEntity>;
    fn group(&self) -> Slvs_hGroup;

    fn val(&self) -> Option<f64> {
        None
    }
    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }
    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
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
