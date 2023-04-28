use super::{AsEntityData, AsLineSegment, Entity, FromSlvsEntity, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_LINE_SEGMENT},
    element::{AsHandle, TypeInfo},
    target::{AsTarget, In3d, OnWorkplane},
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

impl LineSegment<In3d> {
    pub fn new(point_a: Entity<Point<In3d>>, point_b: Entity<Point<In3d>>) -> Self {
        Self {
            workplane: None,
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
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<T: AsTarget> AsLineSegment for LineSegment<T> {}

impl<T: AsTarget> TypeInfo for LineSegment<T> {
    fn type_of() -> String {
        format!("LineSegment<{}>", T::type_of())
    }
}

impl FromSlvsEntity<OnWorkplane> for LineSegment<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            point_a: Entity::new(slvs_entity.point[0]),
            point_b: Entity::new(slvs_entity.point[1]),
        }
    }
}

impl FromSlvsEntity<In3d> for LineSegment<In3d> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: None,
            point_a: Entity::new(slvs_entity.point[0]),
            point_b: Entity::new(slvs_entity.point[1]),
        }
    }
}
