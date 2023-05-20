use crate::entity::{LineSegmentHandle, ProjectionTargetHandle, SomeEntityHandle};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(LineSegmentHandle, ProjectionTargetHandle, SomeEntityHandle)]
pub trait AsHandle {
    fn handle(&self) -> u32;
}
