use super::{AsArc, AsEntityData, Distance, Entity, FromSlvsEntity, Normal, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_CIRCLE},
    element::{AsHandle, AsTarget},
    In3d, OnWorkplane,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle<T: AsTarget> {
    pub workplane: Option<Entity<Workplane>>,
    pub center: Entity<Point<T>>,
    pub radius: Entity<Distance<T>>,
    pub normal: Entity<Normal>,
}

impl Circle<OnWorkplane> {
    pub fn new(
        workplane: Entity<Workplane>,
        center: Entity<Point<OnWorkplane>>,
        radius: Entity<Distance<OnWorkplane>>,
        normal: Entity<Normal>,
    ) -> Self {
        Self {
            workplane: Some(workplane),
            center,
            radius,
            normal,
        }
    }
}

impl Circle<In3d> {
    pub fn new(
        center: Entity<Point<In3d>>,
        radius: Entity<Distance<In3d>>,
        normal: Entity<Normal>,
    ) -> Self {
        Self {
            workplane: None,
            center,
            radius,
            normal,
        }
    }
}

impl<T: AsTarget> AsEntityData for Circle<T> {
    fn type_(&self) -> i32 {
        SLVS_E_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.as_handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.center.as_handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        Some(self.radius.as_handle())
    }
}

impl<T: AsTarget> AsArc for Circle<T> {}

impl FromSlvsEntity<OnWorkplane> for Circle<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            center: Entity::new(slvs_entity.point[0]),
            radius: Entity::new(slvs_entity.distance),
            normal: Entity::new(slvs_entity.normal),
        }
    }
}

impl FromSlvsEntity<In3d> for Circle<In3d> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: None,
            center: Entity::new(slvs_entity.point[0]),
            radius: Entity::new(slvs_entity.distance),
            normal: Entity::new(slvs_entity.normal),
        }
    }
}
