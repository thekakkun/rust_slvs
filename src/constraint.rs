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

use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

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
    element::{AsAny, AsGroup, AsHandle, AsSlvsType, FromSystem},
};

////////////////////////////////////////////////////////////////////////////////
// Constraint Handle
////////////////////////////////////////////////////////////////////////////////

pub(crate) trait AsConstraintHandle: AsHandle {}

impl AsAny for Box<dyn AsConstraintHandle> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl AsConstraintHandle for Box<dyn AsConstraintHandle> {}
impl AsHandle for Box<dyn AsConstraintHandle> {
    fn handle(&self) -> u32 {
        self.as_ref().handle()
    }
}

impl From<Slvs_Constraint> for Box<dyn AsConstraintHandle> {
    fn from(value: Slvs_Constraint) -> Self {
        match value.type_ as _ {
            SLVS_C_POINTS_COINCIDENT => {
                Box::new(ConstraintHandle::<PointsCoincident>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            SLVS_C_PT_PT_DISTANCE => Box::new(ConstraintHandle::<PtPtDistance>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_PT_PLANE_DISTANCE => Box::new(ConstraintHandle::<PtPlaneDistance>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_PT_LINE_DISTANCE => Box::new(ConstraintHandle::<PtLineDistance>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_PT_FACE_DISTANCE => Box::new(ConstraintHandle::<PtFaceDistance>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_PT_IN_PLANE => {
                Box::new(ConstraintHandle::<PtInPlane>::new(value.h)) as Box<dyn AsConstraintHandle>
            }
            SLVS_C_PT_ON_LINE => {
                Box::new(ConstraintHandle::<PtOnLine>::new(value.h)) as Box<dyn AsConstraintHandle>
            }
            SLVS_C_PT_ON_FACE => {
                Box::new(ConstraintHandle::<PtOnFace>::new(value.h)) as Box<dyn AsConstraintHandle>
            }
            SLVS_C_EQUAL_LENGTH_LINES => {
                Box::new(ConstraintHandle::<EqualLengthLines>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            SLVS_C_LENGTH_RATIO => Box::new(ConstraintHandle::<LengthRatio>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_EQ_LEN_PT_LINE_D => Box::new(ConstraintHandle::<EqLenPtLineD>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_EQ_PT_LN_DISTANCES => Box::new(ConstraintHandle::<PtLineDistance>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_EQUAL_ANGLE => Box::new(ConstraintHandle::<EqualAngle>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_EQUAL_LINE_ARC_LEN => {
                Box::new(ConstraintHandle::<EqualLineArcLen>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            SLVS_C_SYMMETRIC => {
                Box::new(ConstraintHandle::<Symmetric>::new(value.h)) as Box<dyn AsConstraintHandle>
            }
            SLVS_C_SYMMETRIC_HORIZ => Box::new(ConstraintHandle::<SymmetricHoriz>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_SYMMETRIC_VERT => Box::new(ConstraintHandle::<SymmetricVert>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_SYMMETRIC_LINE => Box::new(ConstraintHandle::<SymmetricLine>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_AT_MIDPOINT => Box::new(ConstraintHandle::<AtMidpoint>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_HORIZONTAL => Box::new(ConstraintHandle::<Horizontal>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_VERTICAL => {
                Box::new(ConstraintHandle::<Vertical>::new(value.h)) as Box<dyn AsConstraintHandle>
            }
            SLVS_C_SAME_ORIENTATION => Box::new(ConstraintHandle::<SameOrientation>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_ANGLE => {
                Box::new(ConstraintHandle::<Angle>::new(value.h)) as Box<dyn AsConstraintHandle>
            }
            SLVS_C_PARALLEL => {
                Box::new(ConstraintHandle::<Parallel>::new(value.h)) as Box<dyn AsConstraintHandle>
            }
            SLVS_C_PERPENDICULAR => Box::new(ConstraintHandle::<Perpendicular>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_ARC_LINE_TANGENT => Box::new(ConstraintHandle::<ArcLineTangent>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_CUBIC_LINE_TANGENT => {
                Box::new(ConstraintHandle::<CubicLineTangent>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            SLVS_C_WHERE_DRAGGED => Box::new(ConstraintHandle::<WhereDragged>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_LENGTH_DIFFERENCE => {
                Box::new(ConstraintHandle::<LengthDifference>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            SLVS_C_ARC_ARC_LEN_RATIO => Box::new(ConstraintHandle::<ArcArcLenRatio>::new(value.h))
                as Box<dyn AsConstraintHandle>,
            SLVS_C_ARC_LINE_LEN_RATIO => {
                Box::new(ConstraintHandle::<ArcLineLenRatio>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            SLVS_C_ARC_ARC_DIFFERENCE => {
                Box::new(ConstraintHandle::<ArcArcDifference>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            SLVS_C_ARC_LINE_DIFFERENCE => {
                Box::new(ConstraintHandle::<ArcLineDifference>::new(value.h))
                    as Box<dyn AsConstraintHandle>
            }
            _ => panic!("Unknown Slvs_Constraint type value {}", value.type_),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl<C: AsConstraintData> AsConstraintHandle for ConstraintHandle<C> {}
impl<C: AsConstraintData> AsHandle for ConstraintHandle<C> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl<C: AsConstraintData> From<Slvs_Constraint> for ConstraintHandle<C> {
    fn from(value: Slvs_Constraint) -> Self {
        ConstraintHandle::new(value.h)
    }
}

pub trait AsConstraintData: private::Sealed + AsGroup + AsSlvsType + FromSystem {
    fn workplane(&self) -> Option<Slvs_hEntity>;

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
