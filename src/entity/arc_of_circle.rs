use super::{Entity, PointIn2d, Workplane};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcOfCircle {
    pub workplane: Entity<Workplane>,
    pub center: Entity<PointIn2d>,
    pub arc_begin: Entity<PointIn2d>,
    pub arc_end: Entity<PointIn2d>,
}
