use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D},
    element::{AsGroup, AsHandle, AsSlvsType},
    group::Group,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Normal {
    OnWorkplane {
        group: Group,
        workplane: EntityHandle<Workplane>,
    },
    In3d {
        group: Group,
        w: f64,
        x: f64,
        y: f64,
        z: f64,
    },
}

impl AsGroup for Normal {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Self::OnWorkplane { group, .. } => group.handle(),
            Self::In3d { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Normal {
    fn slvs_type(&self) -> i32 {
        match self {
            Self::OnWorkplane { .. } => SLVS_E_NORMAL_IN_2D as _,
            Self::In3d { .. } => SLVS_E_NORMAL_IN_3D as _,
        }
    }
}

impl AsEntityData for Normal {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Self::OnWorkplane { workplane, .. } => Some(workplane.handle()),
            Self::In3d { .. } => None,
        }
    }

    fn param_vals(&self) -> [Option<f64>; 4] {
        match self {
            Self::OnWorkplane { .. } => [None, None, None, None],
            Self::In3d { w, x, y, z, .. } => [Some(*w), Some(*x), Some(*y), Some(*z)],
        }
    }
}
