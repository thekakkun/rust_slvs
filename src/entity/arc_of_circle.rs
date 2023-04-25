use super::{AsArc, AsEntityData, Entity, Normal, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_ARC_OF_CIRCLE},
    element::{AsHandle, OnWorkplane},
    In3d,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcOfCircle {
    pub workplane: Entity<Workplane>,
    pub center: Entity<Point<OnWorkplane>>,
    pub arc_begin: Entity<Point<OnWorkplane>>,
    pub arc_end: Entity<Point<OnWorkplane>>,
    pub normal: Entity<Normal<In3d>>,
}

impl ArcOfCircle {
    pub fn new(
        workplane: Entity<Workplane>,
        center: Entity<Point<OnWorkplane>>,
        arc_begin: Entity<Point<OnWorkplane>>,
        arc_end: Entity<Point<OnWorkplane>>,
        normal: Entity<Normal<In3d>>,
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
