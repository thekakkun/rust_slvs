use super::{AsEntity, Entity, PointIn2d, PointIn3d, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_LINE_SEGMENT, SLVS_FREE_IN_3D},
    AsHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineSegment {
    OnWorkplane {
        workplane: Entity<Workplane>,
        point_a: Entity<PointIn2d>,
        point_b: Entity<PointIn2d>,
    },
    In3d {
        point_a: Entity<PointIn3d>,
        point_b: Entity<PointIn3d>,
    },
}

impl LineSegment {
    pub fn new_on_workplane(
        workplane: Entity<Workplane>,
        point_a: Entity<PointIn2d>,
        point_b: Entity<PointIn2d>,
    ) -> Self {
        Self::OnWorkplane {
            workplane,
            point_a,
            point_b,
        }
    }

    pub fn new_in_3d(point_a: Entity<PointIn3d>, point_b: Entity<PointIn3d>) -> Self {
        Self::In3d { point_a, point_b }
    }
}

impl AsEntity for LineSegment {
    fn type_(&self) -> i32 {
        SLVS_E_LINE_SEGMENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Self::OnWorkplane { workplane, .. } => Some(workplane.as_handle()),
            Self::In3d { .. } => Some(SLVS_FREE_IN_3D),
        }
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(match self {
            Self::OnWorkplane {
                point_a, point_b, ..
            } => vec![point_a.as_handle(), point_b.as_handle()],
            Self::In3d { point_a, point_b } => {
                vec![point_a.as_handle(), point_b.as_handle()]
            }
        })
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}
