use super::{AsEntityData, Entity, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CUBIC},
    element::{AsHandle, AsTarget},
    In3d, OnWorkplane,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cubic<T: AsTarget> {
    pub workplane: Option<Entity<Workplane>>,
    pub start_point: Entity<Point<T>>,
    pub start_control: Entity<Point<T>>,
    pub end_control: Entity<Point<T>>,
    pub end_point: Entity<Point<T>>,
}

impl Cubic<OnWorkplane> {
    pub fn new(
        workplane: Entity<Workplane>,
        start_point: Entity<Point<OnWorkplane>>,
        start_control: Entity<Point<OnWorkplane>>,
        end_control: Entity<Point<OnWorkplane>>,
        end_point: Entity<Point<OnWorkplane>>,
    ) -> Self {
        Self {
            workplane: Some(workplane),
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }
}

impl Cubic<In3d> {
    pub fn new(
        start_point: Entity<Point<In3d>>,
        start_control: Entity<Point<In3d>>,
        end_control: Entity<Point<In3d>>,
        end_point: Entity<Point<In3d>>,
    ) -> Self {
        Self {
            workplane: None,
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }
}

impl<T: AsTarget> AsEntityData for Cubic<T> {
    fn type_(&self) -> i32 {
        SLVS_E_CUBIC as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.as_handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.start_point.as_handle(),
            self.start_control.as_handle(),
            self.end_control.as_handle(),
            self.end_point.as_handle(),
        ])
    }
}
