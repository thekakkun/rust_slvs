use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

/// A point on a workplane or free in 3d.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Point {
    /// A point within a workplane, defined by the workplane and two parameters within
    /// the coordinate system of the workplane.
    OnWorkplane {
        group: Group,
        workplane: EntityHandle<Workplane>,
        coords: [f64; 2],
    },

    /// A point in 3d. Defined by three parameters.
    In3d { group: Group, coords: [f64; 3] },
}

impl Point {
    /// Create a new `Point::OnWorkplane` instance.
    pub fn new_on_workplane(
        group: Group,
        workplane: EntityHandle<Workplane>,
        coords: [f64; 2],
    ) -> Self {
        Self::OnWorkplane {
            group,
            workplane,
            coords,
        }
    }

    /// Create a new `Point::In3d` instance.
    pub fn new_in_3d(group: Group, coords: [f64; 3]) -> Self {
        Self::In3d { group, coords }
    }
}

impl AsGroup for Point {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Point::OnWorkplane { group, .. } => group.handle(),
            Point::In3d { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Point {
    fn slvs_type(&self) -> i32 {
        match self {
            Point::OnWorkplane { .. } => SLVS_E_POINT_IN_2D as _,
            Point::In3d { .. } => SLVS_E_POINT_IN_3D as _,
        }
    }
}

impl AsEntityData for Point {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Point::OnWorkplane { workplane, .. } => Some(workplane.handle()),
            Point::In3d { .. } => None,
        }
    }

    fn param_vals(&self) -> [Option<f64>; 4] {
        match self {
            Point::OnWorkplane { coords: [u, v], .. } => [Some(*u), Some(*v), None, None],
            Point::In3d {
                coords: [x, y, z], ..
            } => [Some(*x), Some(*y), Some(*z), None],
        }
    }
}

impl FromSystem for Point {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;
        let params: Result<Vec<_>, _> = slvs_entity
            .param
            .iter()
            .filter_map(|param_h| match param_h {
                0 => None,
                _ => Some(sys.slvs_param(*param_h)),
            })
            .collect();
        let param_vals: Vec<_> = params?.iter().map(|param| param.val).collect();

        match slvs_entity.type_ as _ {
            SLVS_E_POINT_IN_2D => {
                let coords: [f64; 2] = param_vals
                    .try_into()
                    .map_err(|_| "Expected exactly 2 parameters")?;

                Ok(Self::OnWorkplane {
                    group: Group(slvs_entity.group),
                    workplane: EntityHandle::new(slvs_entity.wrkpl),
                    coords,
                })
            }
            SLVS_E_POINT_IN_3D => {
                let coords: [f64; 3] = param_vals
                    .try_into()
                    .map_err(|_| "Expected exactly 3 parameters")?;

                Ok(Self::In3d {
                    group: Group(slvs_entity.group),
                    coords,
                })
            }
            _ => Err("Expected entity to have type SLVS_E_POINT_IN_2D or SLVS_E_POINT_IN_3D."),
        }
    }
}
