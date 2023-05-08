use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_DISTANCE},
    element::{AsHandle, TypeInfo},
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Distance<T: AsTarget> {
    pub group: Group,
    pub workplane: Option<EntityHandle<Workplane>>,
    pub val: f64,
    phantom: PhantomData<T>,
}

impl Distance<OnWorkplane> {
    pub fn new(group: Group, workplane: EntityHandle<Workplane>, val: f64) -> Self {
        Self {
            group,
            workplane: Some(workplane),
            val,
            phantom: PhantomData,
        }
    }
}

impl Distance<In3d> {
    pub fn new(group: Group, val: f64) -> Self {
        Self {
            group,
            workplane: None,
            val,
            phantom: PhantomData,
        }
    }
}

impl<T: AsTarget> AsEntityData for Distance<T> {
    fn type_(&self) -> i32 {
        SLVS_E_DISTANCE as _
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(vec![self.val])
    }

    fn set_vals(&mut self, vals: Vec<f64>) {
        self.val = vals[0]
    }
}

impl<T: AsTarget> TypeInfo for Distance<T> {
    fn type_of() -> String {
        format!("Distance<{}>", T::type_of())
    }
}

impl<T: AsTarget> From<Slvs_Entity> for Distance<T> {
    fn from(value: Slvs_Entity) -> Self {
        Self {
            group: Group(value.group),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
            val: 0.0,
            phantom: PhantomData,
        }
    }
}

// impl FromSlvsEntity<OnWorkplane> for Distance<OnWorkplane> {
//     fn from(slvs_entity: Slvs_Entity) -> Self {
//         Self {
//             group: Group(slvs_entity.group),
//             workplane: Some(EntityHandle::new(slvs_entity.wrkpl)),
//             val: 0.0,
//             phantom: PhantomData,
//         }
//     }

//     fn set_vals(&mut self, vals: Vec<f64>) {
//         self.val = vals[0]
//     }
// }

// impl FromSlvsEntity<In3d> for Distance<In3d> {
//     fn from(slvs_entity: Slvs_Entity) -> Self {
//         Self {
//             group: Group(slvs_entity.group),
//             workplane: None,
//             val: 0.0,
//             phantom: PhantomData,
//         }
//     }

//     fn set_vals(&mut self, vals: Vec<f64>) {
//         self.val = vals[0]
//     }
// }
