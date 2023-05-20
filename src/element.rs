use crate::entity::{LineSegmentHandle, SomeEntityHandle};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(LineSegmentHandle, SomeEntityHandle)]
pub trait AsHandle {
    fn handle(&self) -> u32;
}
