use super::{Entity, NormalIn3d, PointIn3d};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Workplane {
    pub origin: Entity<PointIn3d>,
    pub normal: Entity<NormalIn3d>,
}
