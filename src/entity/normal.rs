use super::{AsEntityData, Entity, FromSlvsEntity, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D},
    element::{AsElementIdentifier, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Normal {
    OnWorkplane(Entity<Workplane>),
    In3d { w: f64, x: f64, y: f64, z: f64 },
}

impl Normal {
    pub fn new_on_workplane(workplane: Entity<Workplane>) -> Self {
        Self::OnWorkplane(workplane)
    }

    pub fn new_in_3d(quaternion: [f64; 4]) -> Self {
        let [w, x, y, z] = quaternion;
        Self::In3d { w, x, y, z }
    }
}

impl AsEntityData for Normal {
    fn type_(&self) -> i32 {
        match self {
            Self::OnWorkplane { .. } => SLVS_E_NORMAL_IN_2D as _,
            Self::In3d { .. } => SLVS_E_NORMAL_IN_3D as _,
        }
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Self::OnWorkplane(workplane) => Some(workplane.handle()),
            Self::In3d { .. } => None,
        }
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        match self {
            Self::OnWorkplane { .. } => None,
            Self::In3d { w, x, y, z } => Some(vec![*w, *x, *y, *z]),
        }
    }
}

impl FromSlvsEntity<OnWorkplane> for Normal {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        match slvs_entity.wrkpl {
            0 => Self::In3d {
                w: 0.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            h => Self::OnWorkplane(Entity::new(h)),
        }
    }

    fn set_vals(&mut self, vals: Vec<f64>) {
        *self = Normal::In3d {
            w: vals[0],
            x: vals[1],
            y: vals[2],
            z: vals[3],
        }
    }
}
