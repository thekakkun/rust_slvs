use super::{Entity, Workplane};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalIn2d {
    pub workplane: Entity<Workplane>,
}
