use crate::entity::{
    ArcHandle, CubicHandle, CurveHandle, LineSegmentHandle, PointHandle, ProjectionTargetHandle,
    SomeEntityHandle,
};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(
    ArcHandle,
    CubicHandle,
    CurveHandle,
    LineSegmentHandle,
    PointHandle,
    ProjectionTargetHandle,
    SomeEntityHandle
)]
pub trait AsHandle {
    fn handle(&self) -> u32;
}
