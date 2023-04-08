use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

static NEXT_CONSTRAINT_H: AtomicU32 = AtomicU32::new(1);

pub(crate) enum ConstraintType {
    pointsCoincident = binding::SLVS_C_POINTS_COINCIDENT as isize,
    PtPtDistance = binding::SLVS_C_PT_PT_DISTANCE as isize,
    PtPlaneDistance = binding::SLVS_C_PT_PLANE_DISTANCE as isize,
    PtLineDistance = binding::SLVS_C_PT_LINE_DISTANCE as isize,
    PtFaceDistance = binding::SLVS_C_PT_FACE_DISTANCE as isize,
    PtInPlane = binding::SLVS_C_PT_IN_PLANE as isize,
    PtOnLine = binding::SLVS_C_PT_ON_LINE as isize,
    PtOnFace = binding::SLVS_C_PT_ON_FACE as isize,
    EqualLengthLines = binding::SLVS_C_EQUAL_LENGTH_LINES as isize,
    LengthRatio = binding::SLVS_C_LENGTH_RATIO as isize,
    EqLenPtLineD = binding::SLVS_C_EQ_LEN_PT_LINE_D as isize,
    EqPtLnDistances = binding::SLVS_C_EQ_PT_LN_DISTANCES as isize,
    EqualAngle = binding::SLVS_C_EQUAL_ANGLE as isize,
    EqualLineArcLen = binding::SLVS_C_EQUAL_LINE_ARC_LEN as isize,
    Symmetric = binding::SLVS_C_SYMMETRIC as isize,
    SymmetricHoriz = binding::SLVS_C_SYMMETRIC_HORIZ as isize,
    SymmetricVert = binding::SLVS_C_SYMMETRIC_VERT as isize,
    SymmetricLine = binding::SLVS_C_SYMMETRIC_LINE as isize,
    AtMidpoint = binding::SLVS_C_AT_MIDPOINT as isize,
    Horizontal = binding::SLVS_C_HORIZONTAL as isize,
    Vertical = binding::SLVS_C_VERTICAL as isize,
    Diameter = binding::SLVS_C_DIAMETER as isize,
    PtOnCircle = binding::SLVS_C_PT_ON_CIRCLE as isize,
    SameOrientation = binding::SLVS_C_SAME_ORIENTATION as isize,
    Angle = binding::SLVS_C_ANGLE as isize,
    Parallel = binding::SLVS_C_PARALLEL as isize,
    Perpendicular = binding::SLVS_C_PERPENDICULAR as isize,
    ArcLineTangent = binding::SLVS_C_ARC_LINE_TANGENT as isize,
    CubicLineTangent = binding::SLVS_C_CUBIC_LINE_TANGENT as isize,
    EqualRadius = binding::SLVS_C_EQUAL_RADIUS as isize,
    ProjPtDistance = binding::SLVS_C_PROJ_PT_DISTANCE as isize,
    WhereDragged = binding::SLVS_C_WHERE_DRAGGED as isize,
    CurveCurveTangent = binding::SLVS_C_CURVE_CURVE_TANGENT as isize,
    LengthDifference = binding::SLVS_C_LENGTH_DIFFERENCE as isize,
    ArcArcLenRatio = binding::SLVS_C_ARC_ARC_LEN_RATIO as isize,
    ArcLineLenRatio = binding::SLVS_C_ARC_LINE_LEN_RATIO as isize,
    ArcArcDifference = binding::SLVS_C_ARC_ARC_DIFFERENCE as isize,
    ArcLineDifference = binding::SLVS_C_ARC_LINE_DIFFERENCE as isize,
}

pub type Constraint = binding::Slvs_Constraint;

impl Constraint {
    pub(crate) fn new(
        group: binding::Slvs_hGroup,
        type_: ConstraintType,
        wrkpl: Option<binding::Slvs_hEntity>,
        valA: f64,
        pt: [Option<binding::Slvs_hEntity>; 2],
        entity: [Option<binding::Slvs_hEntity>; 4],
        other: [bool; 2],
    ) -> Self {
        Self {
            h: NEXT_CONSTRAINT_H.fetch_add(1, Ordering::SeqCst),
            group,
            type_: type_ as i32,
            wrkpl: wrkpl.unwrap_or(0),
            valA,
            ptA: pt[0].unwrap_or(0),
            ptB: pt[1].unwrap_or(0),
            entityA: entity[0].unwrap_or(0),
            entityB: entity[1].unwrap_or(0),
            entityC: entity[2].unwrap_or(0),
            entityD: entity[3].unwrap_or(0),
            other: other[0].into(),
            other2: other[1].into(),
        }
    }
}