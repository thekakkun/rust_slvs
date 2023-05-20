use serde::{Deserialize, Serialize};

use super::{As2dProjectionTarget, AsEntityData, EntityHandle, SomeEntityHandle, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D},
    element::AsHandle,
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

impl Normal {
    pub fn new_on_workplane(group: Group, workplane: EntityHandle<Workplane>) -> Self {
        Self::OnWorkplane { group, workplane }
    }

    pub fn new_in_3d(group: Group, quaternion: [f64; 4]) -> Self {
        let [w, x, y, z] = quaternion;
        Self::In3d { group, w, x, y, z }
    }
}

impl As2dProjectionTarget for Normal {}

impl AsEntityData for Normal {
    fn into_some_entity_handle(handle: u32) -> SomeEntityHandle {
        SomeEntityHandle::Normal(EntityHandle::new(handle))
    }

    fn from_system(
        sys: &crate::System,
        entity_handle: &EntityHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_entity = sys.slvs_entity(entity_handle.handle())?;

        Ok(match slvs_entity.wrkpl {
            0 => Self::In3d {
                group: Group(slvs_entity.group),
                w: 0.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            h => Self::OnWorkplane {
                group: Group(slvs_entity.group),
                workplane: EntityHandle::new(h),
            },
        })
    }

    fn slvs_type(&self) -> i32 {
        match self {
            Self::OnWorkplane { .. } => SLVS_E_NORMAL_IN_2D as _,
            Self::In3d { .. } => SLVS_E_NORMAL_IN_3D as _,
        }
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Self::OnWorkplane { workplane, .. } => Some(workplane.handle()),
            Self::In3d { .. } => None,
        }
    }

    fn group(&self) -> Slvs_hGroup {
        match self {
            Self::OnWorkplane { group, .. } => group.handle(),
            Self::In3d { group, .. } => group.handle(),
        }
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        match self {
            Self::OnWorkplane { .. } => None,
            Self::In3d { w, x, y, z, .. } => Some(vec![*w, *x, *y, *z]),
        }
    }

    fn set_vals(&mut self, vals: Vec<f64>) {
        if let Normal::In3d { w, x, y, z, .. } = self {
            *w = vals[0];
            *x = vals[1];
            *y = vals[2];
            *z = vals[3];
        }
    }
}

impl From<Slvs_Entity> for Normal {
    fn from(value: Slvs_Entity) -> Self {
        match value.wrkpl {
            0 => Self::In3d {
                group: Group(value.group),
                w: 0.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            h => Self::OnWorkplane {
                group: Group(value.group),
                workplane: EntityHandle::new(h),
            },
        }
    }
}
