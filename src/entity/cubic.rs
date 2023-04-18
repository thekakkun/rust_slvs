use super::{Entity, PointIn2d};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cubic {
    pub point_1: Entity<PointIn2d>,
    pub control_1: Entity<PointIn2d>,
    pub point_2: Entity<PointIn2d>,
    pub control_2: Entity<PointIn2d>,
}
