use serde::{Deserialize, Serialize};

use super::AsEntityData;
use crate::{
    bindings::{Slvs_hGroup, SLVS_E_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    group::Group,
};

define_element!(
    SLVS_E_DISTANCE,
    struct Distance {
        val: f64,
    }
);
impl AsEntityData for Distance {
    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(vec![self.val])
    }
}

// /// An entity used to define the radius of [Circle][crate::entity::Circle]
// ///
// /// See the [module-level documentation][crate] for usage examples.
// #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
// pub struct Distance<T: AsTarget> {
//     pub group: Group,
//     pub workplane: Option<EntityHandle<Workplane>>,
//     /// The value for the circle radius.
//     pub val: f64,
//     /// Holds information on the sketch target to ensure that
//     /// Circle and Distance have the same dimensionality.
//     phantom: PhantomData<T>,
// }

// impl Distance<OnWorkplane> {
//     /// Constructs a new `Distance` on a workplane.
//     pub fn new(group: Group, workplane: EntityHandle<Workplane>, val: f64) -> Self {
//         Self {
//             group,
//             workplane: Some(workplane),
//             val,
//             phantom: PhantomData,
//         }
//     }
// }

// impl Distance<In3d> {
//     /// Constructs a new `Distance` in 3d space.
//     pub fn new(group: Group, val: f64) -> Self {
//         Self {
//             group,
//             workplane: None,
//             val,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<T: AsTarget> AsEntityData for Distance<T> {
//     fn from_system(sys: &System, entity_handle: &EntityHandle<Self>) -> Result<Self, &'static str> {
//         let slvs_entity = sys.slvs_entity(entity_handle.handle())?;
//         let distance = sys.slvs_param(slvs_entity.param[0])?;

//         Ok(Self {
//             group: Group(slvs_entity.group),
//             workplane: match slvs_entity.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//             val: distance.val,
//             phantom: PhantomData,
//         })
//     }

//     fn slvs_type(&self) -> i32 {
//         SLVS_E_DISTANCE as _
//     }

//     fn group(&self) -> Slvs_hGroup {
//         self.group.handle()
//     }

//     fn workplane(&self) -> Option<Slvs_hEntity> {
//         self.workplane.map(|workplane| workplane.handle())
//     }

//     fn param_vals(&self) -> Option<Vec<f64>> {
//         Some(vec![self.val])
//     }
// }
