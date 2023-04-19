use super::{AsEntity, Entity, PointIn2d, PointIn3d, Workplane};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_CUBIC, SLVS_FREE_IN_3D},
    AsHandle,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cubic {
    OnWorkplane {
        workplane: Entity<Workplane>,
        start_point: Entity<PointIn2d>,
        start_control: Entity<PointIn2d>,
        end_control: Entity<PointIn2d>,
        end_point: Entity<PointIn2d>,
    },
    In3d {
        start_point: Entity<PointIn3d>,
        start_control: Entity<PointIn3d>,
        end_control: Entity<PointIn3d>,
        end_point: Entity<PointIn3d>,
    },
}

impl Cubic {
    pub fn new_on_workplane(
        workplane: Entity<Workplane>,
        start_point: Entity<PointIn2d>,
        start_control: Entity<PointIn2d>,
        end_control: Entity<PointIn2d>,
        end_point: Entity<PointIn2d>,
    ) -> Self {
        Self::OnWorkplane {
            workplane,
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }

    pub fn new_in_3d(
        start_point: Entity<PointIn3d>,
        start_control: Entity<PointIn3d>,
        end_control: Entity<PointIn3d>,
        end_point: Entity<PointIn3d>,
    ) -> Self {
        Self::In3d {
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }
}

impl AsEntity for Cubic {
    fn type_(&self) -> i32 {
        SLVS_E_CUBIC as _
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
                start_point,
                start_control,
                end_control,
                end_point,
                ..
            } => vec![
                start_point.as_handle(),
                start_control.as_handle(),
                end_control.as_handle(),
                end_point.as_handle(),
            ],
            Self::In3d {
                start_point,
                start_control,
                end_control,
                end_point,
            } => vec![
                start_point.as_handle(),
                start_control.as_handle(),
                end_control.as_handle(),
                end_point.as_handle(),
            ],
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
