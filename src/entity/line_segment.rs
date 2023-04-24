use super::{AsEntityData, AsLineSegment, Entity, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_LINE_SEGMENT},
    element::{AsHandle, AsTarget},
    In3d, OnWorkplane,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineSegment<T: AsTarget> {
    pub workplane: Option<Entity<Workplane>>,
    pub point_a: Entity<Point<T>>,
    pub point_b: Entity<Point<T>>,
}

impl LineSegment<OnWorkplane> {
    pub fn new(
        workplane: Entity<Workplane>,
        point_a: Entity<Point<OnWorkplane>>,
        point_b: Entity<Point<OnWorkplane>>,
    ) -> Self {
        Self {
            workplane: Some(workplane),
            point_a,
            point_b,
        }
    }
}

impl<T: AsTarget> AsEntityData for LineSegment<T> {
    fn type_(&self) -> i32 {
        SLVS_E_LINE_SEGMENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.as_handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.as_handle(), self.point_b.as_handle()])
    }
}

impl LineSegment<In3d> {
    pub fn new(point_a: Entity<Point<In3d>>, point_b: Entity<Point<In3d>>) -> Self {
        Self {
            workplane: None,
            point_a,
            point_b,
        }
    }
}

impl<T: AsTarget> AsLineSegment for LineSegment<T> {}
