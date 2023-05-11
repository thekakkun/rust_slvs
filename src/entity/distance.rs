use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, SomeEntityHandle, Workplane};
use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_DISTANCE, SLVS_E_POINT_IN_2D,
        SLVS_E_POINT_IN_3D,
    },
    element::AsHandle,
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    fn into_some_entity_handle(handle: u32) -> SomeEntityHandle {
        match T::slvs_type() as _ {
            SLVS_E_POINT_IN_2D => SomeEntityHandle::DistanceOnWorkplane(EntityHandle::new(handle)),
            SLVS_E_POINT_IN_3D => SomeEntityHandle::DistanceIn3d(EntityHandle::new(handle)),
            _ => panic!("Unknown slvs_type {}", T::slvs_type()),
        }
    }

    fn slvs_type(&self) -> i32 {
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
