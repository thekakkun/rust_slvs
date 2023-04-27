use super::{AsEntityData, Entity, FromSlvsEntity, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_CUBIC},
    element::{AsElementIdentifier, AsTarget},
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
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.start_point.handle(),
            self.start_control.handle(),
            self.end_control.handle(),
            self.end_point.handle(),
        ])
    }
}

impl FromSlvsEntity<OnWorkplane> for Cubic<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            start_point: Entity::new(slvs_entity.point[0]),
            start_control: Entity::new(slvs_entity.point[1]),
            end_control: Entity::new(slvs_entity.point[2]),
            end_point: Entity::new(slvs_entity.point[3]),
        }
    }
}

impl FromSlvsEntity<In3d> for Cubic<In3d> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: None,
            start_point: Entity::new(slvs_entity.point[0]),
            start_control: Entity::new(slvs_entity.point[1]),
            end_control: Entity::new(slvs_entity.point[2]),
            end_point: Entity::new(slvs_entity.point[3]),
        }
    }
}
