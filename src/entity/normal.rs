use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

/// A normal on a workplane or free in 3d.
///
/// In SolveSpace, "normals" represent a 3x3 rotation matrix from our base coordinate
/// system to a new frame, defined by the unit quaternion `[w, x, y, z]` where the quaternion
/// is given by `w + x*i + y*j + z*k`.
///
/// It is useful to think of this quaternion as representing a plane through the origin.
/// This plane has three associated vectors: basis vectors `U`, `V` that lie within the
/// plane, and normal `N` that is perpendicular to it.
///
/// Convenience functions are provided to convert between this representation as
/// vectors `U`, `V`, `N` and the unit quaternion.
///
/// A unit quaternion has only 3 degrees of freedom, but is specified in terms of
/// 4 parameters. An extra constraint is therefore generated implicitly, so that
/// `w^2 + x^2 + y^2 + z^2 = 1`
/// See the [module-level documentation][crate] for usage example.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Normal {
    /// A normal within a workplane. This is identical to the workplane's normal.
    OnWorkplane {
        group: Group,
        workplane: EntityHandle<Workplane>,
    },
    /// A `Normal` in 3d space.
    In3d { group: Group, quaternion: [f64; 4] },
}

impl Normal {
    /// Create a new `Normal::OnWorkplane` instance.
    pub fn new_on_workplane(group: Group, workplane: EntityHandle<Workplane>) -> Self {
        Self::OnWorkplane { group, workplane }
    }

    /// Create a new `Normal::In3d` instance.
    pub fn new_in_3d(group: Group, quaternion: [f64; 4]) -> Self {
        Self::In3d { group, quaternion }
    }
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
            Self::In3d { quaternion, .. } => quaternion.map(Some),
        }
    }
}

impl FromSystem for Normal {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
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
                let quaternion: Result<Vec<_>, _> = slvs_entity
                    .param
                    .iter()
                    .map(|param_h| sys.slvs_param(*param_h))
                    .collect();
                Ok(Self::In3d {
                    group: Group(slvs_entity.group),
                    quaternion: quaternion?
                        .iter()
                        .map(|param| param.val)
                        .collect::<Vec<_>>()
                        .try_into()
                        .map_err(|_| "quaternion values not found")?,
                })
            }
            _ => Err("Expected entity to have type SLVS_E_NORMAL_IN_2D or SLVS_E_NORMAL_IN_3D."),
        }
    }
}
