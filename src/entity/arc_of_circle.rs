use super::{AsArc, AsEntityData, Entity, FromSlvsEntity, Normal, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_ARC_OF_CIRCLE},
    element::{AsHandle, TypeInfo},
    target::OnWorkplane,
};

#[derive(Clone, Copy, Debug)]
pub struct ArcOfCircle {
    pub workplane: Entity<Workplane>,
    pub center: Entity<Point<OnWorkplane>>,
    pub arc_begin: Entity<Point<OnWorkplane>>,
    pub arc_end: Entity<Point<OnWorkplane>>,
    pub normal: Entity<Normal>,
}

impl ArcOfCircle {
    pub fn new(
        workplane: Entity<Workplane>,
        center: Entity<Point<OnWorkplane>>,
        arc_begin: Entity<Point<OnWorkplane>>,
        arc_end: Entity<Point<OnWorkplane>>,
        normal: Entity<Normal>,
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

impl AsArc for ArcOfCircle {}

impl AsEntityData for ArcOfCircle {
    fn type_(&self) -> i32 {
        SLVS_E_ARC_OF_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }
    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.center.handle(),
            self.arc_begin.handle(),
            self.arc_end.handle(),
        ])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }
}

impl FromSlvsEntity<OnWorkplane> for ArcOfCircle {
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

impl TypeInfo for ArcOfCircle {
    fn type_of() -> String {
        "ArcOfCircle".to_string()
    }
}
