use super::{
    As2dProjectionTarget, AsEntityData, AsLineSegment, Entity, FromSlvsEntity, Point, Workplane,
};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_LINE_SEGMENT},
    element::{AsHandle, TypeInfo},
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug)]
pub struct LineSegment<T: AsTarget> {
    pub group: Group,
    pub workplane: Option<Entity<Workplane>>,
    pub point_a: Entity<Point<T>>,
    pub point_b: Entity<Point<T>>,
}

impl LineSegment<OnWorkplane> {
    pub fn new(
        group: Group,
        workplane: Entity<Workplane>,
        point_a: Entity<Point<OnWorkplane>>,
        point_b: Entity<Point<OnWorkplane>>,
    ) -> Self {
        Self {
            group,
            workplane: Some(workplane),
            point_a,
            point_b,
        }
    }
}

impl LineSegment<In3d> {
    pub fn new(group: Group, point_a: Entity<Point<In3d>>, point_b: Entity<Point<In3d>>) -> Self {
        Self {
            group,
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

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<T: AsTarget> As2dProjectionTarget for LineSegment<T> {}
impl<T: AsTarget> AsLineSegment for LineSegment<T> {}

impl<T: AsTarget> TypeInfo for LineSegment<T> {
    fn type_of() -> String {
        format!("LineSegment<{}>", T::type_of())
    }
}

impl FromSlvsEntity<OnWorkplane> for LineSegment<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            group: Group(slvs_entity.group),
            workplane: Some(Entity::new(slvs_entity.wrkpl)),
            point_a: Entity::new(slvs_entity.point[0]),
            point_b: Entity::new(slvs_entity.point[1]),
        }
    }
}

impl FromSlvsEntity<In3d> for LineSegment<In3d> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            group: Group(slvs_entity.group),
            workplane: None,
            point_a: Entity::new(slvs_entity.point[0]),
            point_b: Entity::new(slvs_entity.point[1]),
        }
    }
}
