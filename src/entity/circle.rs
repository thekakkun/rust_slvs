use super::{Distance, Entity, NormalIn2d, PointIn2d};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub normal: Entity<NormalIn2d>,
    pub center: Entity<PointIn2d>,
    pub radius: Entity<Distance>,
}
