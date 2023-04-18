use super::{Entity, Workplane};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointIn2d {
    pub workplane: Entity<Workplane>,
    pub u: f64,
    pub v: f64,
}
