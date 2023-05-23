use crate::{
    constraint::SomeConstraintHandle,
    entity::{
        ArcHandle, CubicHandle, CurveHandle, LineSegmentHandle, PointHandle,
        ProjectionTargetHandle, SomeEntityHandle,
    },
};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(
    ArcHandle,
    CubicHandle,
    CurveHandle,
    LineSegmentHandle,
    PointHandle,
    ProjectionTargetHandle,
    SomeEntityHandle,
    SomeConstraintHandle
)]
pub trait AsHandle: private::Sealed {
    fn handle(&self) -> u32;
}

mod private {
    use super::AsHandle;

    pub trait Sealed {}
    impl<H: AsHandle> Sealed for H {}
}
