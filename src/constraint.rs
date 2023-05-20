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
pub use horizontal::{LineHorizontal, PointsHorizontal};
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
pub use vertical::{LineVertical, PointsVertical};
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
use std::{any::type_name, fmt::Debug, marker::PhantomData};

use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup},
    element::AsHandle,
};

////////////////////////////////////////////////////////////////////////////////
// Constraint Handle
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ConstraintHandle<T: AsConstraintData> {
    pub handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsConstraintData> ConstraintHandle<T> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsConstraintData> Debug for ConstraintHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Constraint: {{h: {}, type: {}}}",
            self.handle,
            type_name::<T>()
        )
    }
}

impl<T: AsConstraintData> AsHandle for ConstraintHandle<T> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity Data
////////////////////////////////////////////////////////////////////////////////

pub trait AsConstraintData: Copy + Debug {
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
