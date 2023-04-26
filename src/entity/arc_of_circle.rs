use super::{AsArc, AsEntityData, AsNormal, Entity, FromSlvsEntity, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_ARC_OF_CIRCLE},
    element::{AsHandle, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcOfCircle<N: AsNormal> {
    pub workplane: Entity<Workplane>,
    pub center: Entity<Point<OnWorkplane>>,
    pub arc_begin: Entity<Point<OnWorkplane>>,
    pub arc_end: Entity<Point<OnWorkplane>>,
    pub normal: Entity<N>,
}

impl<N: AsNormal> ArcOfCircle<N> {
    pub fn new(
        workplane: Entity<Workplane>,
        center: Entity<Point<OnWorkplane>>,
        arc_begin: Entity<Point<OnWorkplane>>,
        arc_end: Entity<Point<OnWorkplane>>,
        normal: Entity<N>,
    ) -> Self {
        Self {
            workplane,
            center,
            arc_begin,
            arc_end,
            normal,
        }
    }
}

impl<N: AsNormal> AsArc for ArcOfCircle<N> {}

impl<N: AsNormal> AsEntityData for ArcOfCircle<N> {
    fn type_(&self) -> i32 {
        SLVS_E_ARC_OF_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.as_handle())
    }
    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.center.as_handle(),
            self.arc_begin.as_handle(),
            self.arc_end.as_handle(),
        ])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }
}

impl<N: AsNormal> FromSlvsEntity<OnWorkplane> for ArcOfCircle<N> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            workplane: Entity::new(slvs_entity.wrkpl),
            center: Entity::new(slvs_entity.point[0]),
            arc_begin: Entity::new(slvs_entity.point[1]),
            arc_end: Entity::new(slvs_entity.point[2]),
            normal: Entity::new(slvs_entity.normal),
        }
    }
}
