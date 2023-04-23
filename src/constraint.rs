use std::marker::PhantomData;

use crate::{
    bindings::{
        Slvs_hConstraint, Slvs_hEntity, SLVS_C_ANGLE, SLVS_C_ARC_ARC_DIFFERENCE,
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
    element::{AsHandle, Target},
    In3d, OnWorkplane,
};

mod pt_pt_distance;
pub use pt_pt_distance::PtPtDistance;
// mod pt_line_distance;
// pub use pt_line_distance::PtLineDistance;

////////////////////////////////////////////////////////////////////////////////
// Constraint of a specific type
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraint<T: AsConstraintData, U: Target> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<(T, U)>,
}

impl<T: AsConstraintData, U: Target> Constraint<T, U> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsConstraintData, U: Target> AsHandle for Constraint<T, U> {
    fn as_handle(&self) -> u32 {
        self.handle
    }
}

pub trait AsConstraintData {
    fn type_(&self) -> i32;
    fn val(&self) -> Option<f64>;
    fn points(&self) -> Option<Vec<Slvs_hEntity>>;
    fn entities(&self) -> Option<Vec<Slvs_hEntity>>;
    fn others(&self) -> [bool; 2];
}

// impl<T, U> From<SomeConstraint> for Constraint<T>
// where
//     T: AsConstraint<Apply = U> + 'static,
//     U: Target + 'static,
// {
//     fn from(value: SomeConstraint) -> Self {
//         match (value.type_ as _, value.workplane) {
//             // SLVS_C_POINTS_COINCIDENT => todo!(),
//             (SLVS_C_PT_PT_DISTANCE, 0) => Self {
//                 handle: value.handle,
//                 phantom: PhantomData::<PtPtDistance<OnWorkplane>>,
//             },
//             (SLVS_C_PT_PT_DISTANCE, _) => {
//                 Constraint::<PtPtDistance<OnWorkplane>>::new(value.handle)
//             }
//             // SLVS_C_PT_PLANE_DISTANCE => todo!(),
//             (SLVS_C_PT_LINE_DISTANCE, 0) => Self::new(value.handle),
//             (SLVS_C_PT_LINE_DISTANCE, _) => Self::new(value.handle),
//             // SLVS_C_PT_FACE_DISTANCE => todo!(),
//             // SLVS_C_PT_IN_PLANE => todo!(),
//             // SLVS_C_PT_ON_LINE => todo!(),
//             // SLVS_C_PT_ON_FACE => todo!(),
//             // SLVS_C_EQUAL_LENGTH_LINES => todo!(),
//             // SLVS_C_LENGTH_RATIO => todo!(),
//             // SLVS_C_EQ_LEN_PT_LINE_D => todo!(),
//             // SLVS_C_EQ_PT_LN_DISTANCES => todo!(),
//             // SLVS_C_EQUAL_ANGLE => todo!(),
//             // SLVS_C_EQUAL_LINE_ARC_LEN => todo!(),
//             // SLVS_C_SYMMETRIC => todo!(),
//             // SLVS_C_SYMMETRIC_HORIZ => todo!(),
//             // SLVS_C_SYMMETRIC_VERT => todo!(),
//             // SLVS_C_SYMMETRIC_LINE => todo!(),
//             // SLVS_C_AT_MIDPOINT => todo!(),
//             // SLVS_C_HORIZONTAL => todo!(),
//             // SLVS_C_VERTICAL => todo!(),
//             // SLVS_C_DIAMETER => todo!(),
//             // SLVS_C_PT_ON_CIRCLE => todo!(),
//             // SLVS_C_SAME_ORIENTATION => todo!(),
//             // SLVS_C_ANGLE => todo!(),
//             // SLVS_C_PARALLEL => todo!(),
//             // SLVS_C_PERPENDICULAR => todo!(),
//             // SLVS_C_ARC_LINE_TANGENT => todo!(),
//             // SLVS_C_CUBIC_LINE_TANGENT => todo!(),
//             // SLVS_C_EQUAL_RADIUS => todo!(),
//             // SLVS_C_PROJ_PT_DISTANCE => todo!(),
//             // SLVS_C_WHERE_DRAGGED => todo!(),
//             // SLVS_C_CURVE_CURVE_TANGENT => todo!(),
//             // SLVS_C_LENGTH_DIFFERENCE => todo!(),
//             // SLVS_C_ARC_ARC_LEN_RATIO => todo!(),
//             // SLVS_C_ARC_LINE_LEN_RATIO => todo!(),
//             // SLVS_C_ARC_ARC_DIFFERENCE => todo!(),
//             // SLVS_C_ARC_LINE_DIFFERENCE => todo!(),
//             _ => panic!("Unknown constraint type: {}", value.type_),
//         }
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SomeConstraint {
    pub(super) handle: u32,
}

impl SomeConstraint {
    pub fn new(handle: u32) -> Self {
        Self { handle }
    }
}

impl AsHandle for SomeConstraint {
    fn as_handle(&self) -> u32 {
        self.handle
    }
}

// // #[derive(Clone, Copy, Debug, PartialEq)]
// pub enum SomeConstraint {
//     // PointsCoincident(Constraint<PointsCoincident>),
//     PtPtDistance(Constraint<PtPtDistance<dyn Target>>),
//     // PtPlaneDistance(Constraint<PtPlaneDistance>),
//     // PtLineDistance(Constraint<PtLineDistance>),
//     // PtFaceDistance(Constraint<PtFaceDistance>),
//     // PtInPlane(Constraint<PtInPlane>),
//     // PtOnLine(Constraint<PtOnLine>),
//     // PtOnFace(Constraint<PtOnFace>),
//     // EqualLengthLines(Constraint<EqualLengthLines>),
//     // LengthRatio(Constraint<LengthRatio>),
//     // EqLenPtLineD(Constraint<EqLenPtLineD>),
//     // EqPtLnDistances(Constraint<EqPtLnDistances>),
//     // EqualAngle(Constraint<EqualAngle>),
//     // EqualLineArcLen(Constraint<EqualLineArcLen>),
//     // Symmetric(Constraint<Symmetric>),
//     // SymmetricHoriz(Constraint<SymmetricHoriz>),
//     // SymmetricVert(Constraint<SymmetricVert>),
//     // SymmetricLine(Constraint<SymmetricLine>),
//     // AtMidpoint(Constraint<AtMidpoint>),
//     // Horizontal(Constraint<Horizontal>),
//     // Vertical(Constraint<Vertical>),
//     // Diameter(Constraint<Diameter>),
//     // PtOnCircle(Constraint<PtOnCircle>),
//     // SameOrientation(Constraint<SameOrientation>),
//     // Angle(Constraint<Angle>),
//     // Parallel(Constraint<Parallel>),
//     // Perpendicular(Constraint<Perpendicular>),
//     // ArcLineTangent(Constraint<ArcLineTangent>),
//     // CubicLineTangent(Constraint<CubicLineTangent>),
//     // EqualRadius(Constraint<EqualRadius>),
//     // ProjPtDistance(Constraint<ProjPtDistance>),
//     // WhereDragged(Constraint<WhereDragged>),
//     // CurveCurveTangent(Constraint<CurveCurveTangent>),
//     // LengthDifference(Constraint<LengthDifference>),
//     // ArcArcLenRatio(Constraint<ArcArcLenRatio>),
//     // ArcLineLenRatio(Constraint<ArcLineLenRatio>),
//     // ArcArcDifference(Constraint<ArcArcDifference>),
//     // ArcLineDifference(Constraint<ArcLineDifference>),
// }

// impl SomeConstraint {
//     pub(super) fn new(type_: u32, h: Slvs_hConstraint) -> Self {
//         match type_ {
//             SLVS_C_POINTS_COINCIDENT => todo!(),
//             SLVS_C_PT_PT_DISTANCE => SomeConstraint::PtPtDistance(Constraint::new(h)),
//             SLVS_C_PT_PLANE_DISTANCE => todo!(),
//             SLVS_C_PT_LINE_DISTANCE => todo!(),
//             SLVS_C_PT_FACE_DISTANCE => todo!(),
//             SLVS_C_PT_IN_PLANE => todo!(),
//             SLVS_C_PT_ON_LINE => todo!(),
//             SLVS_C_PT_ON_FACE => todo!(),
//             SLVS_C_EQUAL_LENGTH_LINES => todo!(),
//             SLVS_C_LENGTH_RATIO => todo!(),
//             SLVS_C_EQ_LEN_PT_LINE_D => todo!(),
//             SLVS_C_EQ_PT_LN_DISTANCES => todo!(),
//             SLVS_C_EQUAL_ANGLE => todo!(),
//             SLVS_C_EQUAL_LINE_ARC_LEN => todo!(),
//             SLVS_C_SYMMETRIC => todo!(),
//             SLVS_C_SYMMETRIC_HORIZ => todo!(),
//             SLVS_C_SYMMETRIC_VERT => todo!(),
//             SLVS_C_SYMMETRIC_LINE => todo!(),
//             SLVS_C_AT_MIDPOINT => todo!(),
//             SLVS_C_HORIZONTAL => todo!(),
//             SLVS_C_VERTICAL => todo!(),
//             SLVS_C_DIAMETER => todo!(),
//             SLVS_C_PT_ON_CIRCLE => todo!(),
//             SLVS_C_SAME_ORIENTATION => todo!(),
//             SLVS_C_ANGLE => todo!(),
//             SLVS_C_PARALLEL => todo!(),
//             SLVS_C_PERPENDICULAR => todo!(),
//             SLVS_C_ARC_LINE_TANGENT => todo!(),
//             SLVS_C_CUBIC_LINE_TANGENT => todo!(),
//             SLVS_C_EQUAL_RADIUS => todo!(),
//             SLVS_C_PROJ_PT_DISTANCE => todo!(),
//             SLVS_C_WHERE_DRAGGED => todo!(),
//             SLVS_C_CURVE_CURVE_TANGENT => todo!(),
//             SLVS_C_LENGTH_DIFFERENCE => todo!(),
//             SLVS_C_ARC_ARC_LEN_RATIO => todo!(),
//             SLVS_C_ARC_LINE_LEN_RATIO => todo!(),
//             SLVS_C_ARC_ARC_DIFFERENCE => todo!(),
//             SLVS_C_ARC_LINE_DIFFERENCE => todo!(),
//             _ => panic!("Unknown constraint type: {}", type_),
//         }
//     }
// }
