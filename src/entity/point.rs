use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    element::{AsGroup, AsHandle, AsSlvsType},
    group::Group,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Point {
    OnWorkplane {
        group: Group,
        workplane: EntityHandle<Workplane>,
        coords: [f64; 2],
    },
    In3d {
        group: Group,
        coords: [f64; 3],
    },
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

    fn param_vals(&self) -> Option<Vec<f64>> {
        match self {
            Point::OnWorkplane { coords, .. } => Some((*coords).into()),
            Point::In3d { coords, .. } => Some((*coords).into()),
        }
    }
}

// #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
// pub struct Point<T: AsTarget> {
//     pub group: Group,
//     pub workplane: Option<EntityHandle<Workplane>>,
//     pub coords: T,
// }

// impl Point<OnWorkplane> {
//     pub fn new(group: Group, workplane: EntityHandle<Workplane>, u: f64, v: f64) -> Self {
//         Self {
//             group,
//             workplane: Some(workplane),
//             coords: OnWorkplane(u, v),
//         }
//     }
// }

// impl Point<In3d> {
//     pub fn new(group: Group, x: f64, y: f64, z: f64) -> Self {
//         Self {
//             group,
//             workplane: None,
//             coords: In3d(x, y, z),
//         }
//     }
// }

// impl<T: AsTarget> AsEntityData for Point<T> {
//     fn from_system(sys: &System, entity_handle: &EntityHandle<Self>) -> Result<Self, &'static str> {
//         let slvs_entity = sys.slvs_entity(entity_handle.handle())?;
//         let slvs_params: Result<Vec<_>, _> = slvs_entity
//             .param
//             .iter()
//             .filter_map(|param_h| match param_h {
//                 0 => None,
//                 h => Some(sys.slvs_param(*h)),
//             })
//             .collect();
//         let param_vals: Vec<_> = slvs_params?
//             .iter()
//             .map(|slvs_param| slvs_param.val)
//             .collect();

//         match T::slvs_type() as _ {
//             SLVS_E_POINT_IN_2D => Ok(Self {
//                 group: Group(slvs_entity.group),
//                 workplane: Some(EntityHandle::new(slvs_entity.wrkpl)),
//                 coords: param_vals.try_into()?,
//             }),
//             SLVS_E_POINT_IN_3D => Ok(Self {
//                 group: Group(slvs_entity.group),
//                 workplane: Some(EntityHandle::new(slvs_entity.wrkpl)),
//                 coords: param_vals.try_into()?,
//             }),
//             _ => panic!("Type should be for point in 2d or 3d"),
//         }
//     }

//     fn slvs_type(&self) -> i32 {
//         T::slvs_type()
//     }

//     fn workplane(&self) -> Option<Slvs_hEntity> {
//         self.workplane.map(|w| w.handle())
//     }

//     fn group(&self) -> Slvs_hGroup {
//         self.group.handle()
//     }

//     fn param_vals(&self) -> Option<Vec<f64>> {
//         Some(self.coords.into())
//     }
// }
