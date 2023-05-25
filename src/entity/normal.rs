use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
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

impl FromSystem for Normal {
    fn from_system(sys: &crate::System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;

        match slvs_entity.type_ as _ {
            SLVS_E_NORMAL_IN_2D => Ok(Self::OnWorkplane {
                group: Group(slvs_entity.group),
                workplane: EntityHandle::new(slvs_entity.wrkpl),
            }),
            SLVS_E_NORMAL_IN_3D => {
                let [w, x, y, z] = slvs_entity.param.map(|param_h| sys.slvs_param(param_h));
                Ok(Self::In3d {
                    group: Group(slvs_entity.group),
                    w: w?.val,
                    x: x?.val,
                    y: y?.val,
                    z: z?.val,
                })
            }
            _ => Err("Expected entity to have type SLVS_E_NORMAL_IN_2D or SLVS_E_NORMAL_IN_3D"),
        }
    }
}
