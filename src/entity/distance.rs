use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use super::{AsEntityData, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_DISTANCE},
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
    fn from_system(
        sys: &crate::System,
        entity_handle: &EntityHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_entity = sys.slvs_entity(entity_handle.handle())?;
        let distance = sys.slvs_param(slvs_entity.param[0])?;

        Ok(Self {
            group: Group(slvs_entity.group),
            workplane: match slvs_entity.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
            val: distance.val,
            phantom: PhantomData,
        })
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
}
