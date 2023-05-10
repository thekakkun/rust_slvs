use serde::{Deserialize, Serialize};

use super::{AsEntityData, AsPoint, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    element::AsHandle,
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point<T: AsTarget> {
    pub group: Group,
    pub workplane: Option<EntityHandle<Workplane>>,
    pub coords: T,
}

impl Point<OnWorkplane> {
    pub fn new(group: Group, workplane: EntityHandle<Workplane>, u: f64, v: f64) -> Self {
        Self {
            group,
            workplane: Some(workplane),
            coords: OnWorkplane(u, v),
        }
    }
}

impl Point<In3d> {
    pub fn new(group: Group, x: f64, y: f64, z: f64) -> Self {
        Self {
            group,
            workplane: None,
            coords: In3d(x, y, z),
        }
    }
}

impl<T: AsTarget> AsPoint for Point<T> {}

impl<T: AsTarget> AsEntityData for Point<T> {
    fn into_some_entity_handle(handle: u32) -> super::SomeEntityHandle {
        T::into_some_entity_handle(handle)
    }

    fn slvs_type(&self) -> i32 {
        T::slvs_type()
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|w| w.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(self.coords.into())
    }

    fn set_vals(&mut self, vals: Vec<f64>) {
        self.coords = vals.into();
    }
}

impl<T: AsTarget + Default> From<Slvs_Entity> for Point<T> {
    fn from(value: Slvs_Entity) -> Self {
        if value.type_ == SLVS_E_POINT_IN_2D as _ {
            Self {
                group: Group(value.group),
                workplane: Some(EntityHandle::new(value.wrkpl)),
                coords: T::default(),
            }
        } else {
            Self {
                group: Group(value.group),
                workplane: None,
                coords: T::default(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PointHandle {
    OnWorkplane(EntityHandle<Point<OnWorkplane>>),
    In3d(EntityHandle<Point<In3d>>),
}

impl AsHandle for PointHandle {
    fn handle(&self) -> u32 {
        match self {
            Self::OnWorkplane(entity_handle) => entity_handle.handle(),
            Self::In3d(entity_handle) => entity_handle.handle(),
        }
    }
}

impl TryFrom<Slvs_Entity> for PointHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        match value.type_ as _ {
            SLVS_E_POINT_IN_2D => Ok(PointHandle::OnWorkplane(value.into())),
            SLVS_E_POINT_IN_3D => Ok(PointHandle::In3d(value.into())),
            _ => Err("Unexpected Slvs_Entity type"),
        }
    }
}
