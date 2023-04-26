use super::{AsArc, AsEntityData, AsNormal, Distance, Entity, FromSlvsEntity, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_CIRCLE},
    element::{AsHandle, AsTarget},
    In3d, OnWorkplane,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle<T: AsTarget, N: AsNormal> {
    pub workplane: Option<Entity<Workplane>>,
    pub center: Entity<Point<T>>,
    pub radius: Entity<Distance<T>>,
    pub normal: Entity<N>,
}

impl<N: AsNormal> Circle<OnWorkplane, N> {
    pub fn new(
        workplane: Entity<Workplane>,
        center: Entity<Point<OnWorkplane>>,
        radius: Entity<Distance<OnWorkplane>>,
        normal: Entity<N>,
    ) -> Self {
        Self {
            workplane: Some(workplane),
            center,
            radius,
            normal,
        }
    }
}

impl<N: AsNormal> Circle<In3d, N> {
    pub fn new(
        center: Entity<Point<In3d>>,
        radius: Entity<Distance<In3d>>,
        normal: Entity<N>,
    ) -> Self {
        Self {
            workplane: None,
            center,
            radius,
            normal,
        }
    }
}

impl<T: AsTarget, N: AsNormal> AsEntityData for Circle<T, N> {
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

impl<T: AsTarget, N: AsNormal> AsArc for Circle<T, N> {}

impl<N: AsNormal> FromSlvsEntity<OnWorkplane> for Circle<OnWorkplane, N> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            center: Entity::new(slvs_entity.point[0]),
            radius: Entity::new(slvs_entity.distance),
            normal: Entity::new(slvs_entity.normal),
        }
    }
}

impl<N: AsNormal> FromSlvsEntity<In3d> for Circle<In3d, N> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: None,
            center: Entity::new(slvs_entity.point[0]),
            radius: Entity::new(slvs_entity.distance),
            normal: Entity::new(slvs_entity.normal),
        }
    }
}
