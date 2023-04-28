use std::{fmt::Debug, marker::PhantomData};

use crate::{
    bindings::Slvs_hEntity,
    element::{AsHandle, TypeInfo},
};

mod points_coincident;
pub use points_coincident::PointsCoincident;
mod pt_pt_distance;
pub use pt_pt_distance::PtPtDistance;
mod pt_plane_distance;
pub use pt_plane_distance::PtPlaneDistance;
mod pt_line_distance;
pub use pt_line_distance::PtLineDistance;
mod pt_face_distance;
pub use pt_face_distance::PtFaceDistance;
mod pt_in_plane;
pub use pt_in_plane::PtInPlane;
mod pt_on_line;
pub use pt_on_line::PtOnLine;
mod pt_on_face;
pub use pt_on_face::PtOnFace;
mod equal_length_lines;
pub use equal_length_lines::EqualLengthLines;
mod length_ratio;
pub use length_ratio::LengthRatio;
mod eq_len_pt_line_d;
pub use eq_len_pt_line_d::EqLenPtLineD;
mod eq_pt_ln_distances;
pub use eq_pt_ln_distances::EqPtLnDistances;
mod equal_angle;
pub use equal_angle::EqualAngle;
mod equal_line_arc_len;
pub use equal_line_arc_len::EqualLineArcLen;
// mod symmetric             ;
// mod symmetric_horiz       ;
// mod symmetric_vert        ;
// mod symmetric_line        ;
// mod at_midpoint           ;
mod horizontal;
pub use horizontal::{LineHorizontal, PointsHorizontal};
mod vertical;
pub use vertical::{LineVertical, PointsVertical};
mod diameter;
pub use diameter::Diameter;
// mod pt_on_circle          ;
// mod same_orientation      ;
// mod angle                 ;
// mod parallel              ;
// mod perpendicular         ;
// mod arc_line_tangent      ;
// mod cubic_line_tangent    ;
mod equal_radius;
pub use equal_radius::EqualRadius;
// mod proj_pt_distance      ;
// mod where_dragged         ;
// mod curve_curve_tangent   ;
// mod length_difference     ;
// mod arc_arc_len_ratio     ;
// mod arc_line_len_ratio    ;
// mod arc_arc_difference    ;
// mod arc_line_difference   ;

pub trait AsConstraint: AsHandle {
    fn clone_dyn(&self) -> Box<dyn AsConstraint>;
}

impl Clone for Box<dyn AsConstraint> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

pub trait AsConstraintData: Copy + Debug + TypeInfo {
    fn type_(&self) -> i32;
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

#[derive(Clone, Copy)]
pub struct Constraint<T: AsConstraintData> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsConstraintData> Constraint<T> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsConstraintData> Debug for Constraint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Constraint: {{h: {}, type: {}}}",
            self.handle,
            T::type_of()
        )
    }
}

impl<T: AsConstraintData + 'static> AsConstraint for Constraint<T> {
    fn clone_dyn(&self) -> Box<dyn AsConstraint> {
        Box::new(*self)
    }
}

impl<T: AsConstraintData> AsHandle for Constraint<T> {
    fn handle(&self) -> u32 {
        self.handle
    }
}
