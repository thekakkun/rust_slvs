use std::marker::PhantomData;

use crate::binding;

pub mod pt_pt_distance;
pub use pt_pt_distance::PtPtDistance;

pub trait AsConstraint {
    fn type_(&self) -> binding::Slvs_hConstraint;
    fn workplane(&self) -> Option<binding::Slvs_hEntity>;
    fn val(&self) -> f64;
    fn point(&self) -> [Option<binding::Slvs_hEntity>; 2];
    fn entity(&self) -> [Option<binding::Slvs_hEntity>; 4];
    fn other(&self) -> [bool; 2];
}

#[derive(Clone, Copy)]
pub struct Constraint<T: AsConstraint + ?Sized> {
    pub(super) handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsConstraint + ?Sized> Constraint<T> {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsConstraint> From<Constraint<T>> for binding::Slvs_hConstraint {
    fn from(value: Constraint<T>) -> Self {
        value.handle
    }
}

pub enum SomeConstraint {
    PtPtDistance(Constraint<PtPtDistance>),
    // PointsCoincident(),
    // PtPlaneDistance(),
    // PtLineDistance(),
    // PtFaceDistance(),
    // PtInPlane(),
    // PtOnLine(),
    // PtOnFace(),
    // EqualLengthLines(),
    // LengthRatio(),
    // EqLenPtLineD(),
    // EqPtLnDistances(),
    // EqualAngle(),
    // EqualLineArcLen(),
    // Symmetric(),
    // SymmetricHoriz(),
    // SymmetricVert(),
    // SymmetricLine(),
    // AtMidpoint(),
    // Horizontal(),
    // Vertical(),
    // Diameter(),
    // PtOnCircle(),
    // SameOrientation(),
    // Angle(),
    // Parallel(),
    // Perpendicular(),
    // ArcLineTangent(),
    // CubicLineTangent(),
    // EqualRadius(),
    // ProjPtDistance(),
    // WhereDragged(),
    // CurveCurveTangent(),
    // LengthDifference(),
    // ArcArcLenRatio(),
    // ArcLineLenRatio(),
    // ArcArcDifference(),
    // ArcLineDifference(),
}
