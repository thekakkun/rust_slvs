use std::marker::PhantomData;

use crate::{bindings::Slvs_hEntity, element::AsHandle};

// mod points_coincident     ;
mod pt_pt_distance;
pub use pt_pt_distance::PtPtDistance;
// mod pt_plane_distance     ;
mod pt_line_distance;
pub use pt_line_distance::PtLineDistance;
// mod pt_face_distance      ;
// mod pt_in_plane           ;
// mod pt_on_line            ;
// mod pt_on_face            ;
// mod equal_length_lines    ;
// mod length_ratio          ;
// mod eq_len_pt_line_d      ;
// mod eq_pt_ln_distances    ;
// mod equal_angle           ;
// mod equal_line_arc_len    ;
// mod symmetric             ;
// mod symmetric_horiz       ;
// mod symmetric_vert        ;
// mod symmetric_line        ;
// mod at_midpoint           ;
mod horizontal;
pub use horizontal::{HorizontalLine, HorizontalPoints};
mod vertical;
pub use vertical::{VerticalLine, VerticalPoints};
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

pub trait AsConstraintData {
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl<T: AsConstraintData> AsHandle for Constraint<T> {
    fn as_handle(&self) -> u32 {
        self.handle
    }
}
