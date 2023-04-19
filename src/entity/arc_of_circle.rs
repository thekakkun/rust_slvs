use super::{AsEntity, Entity, NormalIn2d, PointIn2d, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_ARC_OF_CIRCLE},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcOfCircle {
    pub workplane: Entity<Workplane>,
    pub center: Entity<PointIn2d>,
    pub arc_begin: Entity<PointIn2d>,
    pub arc_end: Entity<PointIn2d>,
    pub normal: Entity<NormalIn2d>,
}

impl ArcOfCircle {
    pub fn new(
        workplane: Entity<Workplane>,
        center: Entity<PointIn2d>,
        arc_begin: Entity<PointIn2d>,
        arc_end: Entity<PointIn2d>,
        normal: Entity<NormalIn2d>,
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

impl AsEntity for ArcOfCircle {
    fn type_(&self) -> i32 {
        SLVS_E_ARC_OF_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.as_handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.arc_begin.as_handle(),
            self.arc_begin.as_handle(),
            self.arc_end.as_handle(),
        ])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
