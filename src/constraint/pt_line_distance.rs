use std::marker::PhantomData;

use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_LINE_DISTANCE},
    element::{AsHandle, Target},
    entity::{Entity, LineSegment, Point},
};

use super::AsConstraint;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PtLineDistance<T: Target + ?Sized> {
    point: Entity<Point<dyn Target>>,
    line: Entity<LineSegment<dyn Target>>,
    distance: f64,
    phantom: PhantomData<T>,
}

impl<T: Target + ?Sized> PtLineDistance<T> {
    pub fn new(
        point: Entity<Point<dyn Target>>,
        line: Entity<LineSegment<dyn Target>>,
        distance: f64,
    ) -> Self {
        Self {
            point,
            line,
            distance,
            phantom: PhantomData,
        }
    }
}

impl<T: Target + ?Sized> AsConstraint for PtLineDistance<T> {
    type Apply = T;

    fn type_(&self) -> i32 {
        SLVS_C_PT_LINE_DISTANCE as _
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.as_handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }

    fn others(&self) -> [bool; 2] {
        [false, false]
    }
}
