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
pub trait AsHandle {
    fn handle(&self) -> u32;
}
