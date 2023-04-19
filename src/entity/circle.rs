use super::{AsEntity, Distance, Entity, NormalIn2d, NormalIn3d, PointIn2d, PointIn3d, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CIRCLE},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Circle {
    OnWorkplane {
        workplane: Entity<Workplane>,
        center: Entity<PointIn2d>,
        radius: Entity<Distance>,
        normal: Entity<NormalIn2d>,
    },
    In3d {
        center: Entity<PointIn3d>,
        radius: Entity<Distance>,
        normal: Entity<NormalIn3d>,
    },
}

impl AsEntity for Circle {
    fn type_(&self) -> i32 {
        SLVS_E_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
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

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
