use serde::{Deserialize, Serialize};

use super::{AsEntityData, AsPoint, EntityHandle, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup},
    element::AsHandle,
    group::Group,
    target::{AsTarget, In3d, OnWorkplane, Target},
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

    fn from_system(
        sys: &crate::System,
        entity_handle: &EntityHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_entity = sys.slvs_entity(entity_handle.handle())?;
        let param_handle_iter = slvs_entity.param.iter();

        match T::target_type() as _ {
            Target::OnWorkplane => {
                let points: Result<Vec<_>, _> = param_handle_iter
                    .take(2)
                    .map(|param_h| {
                        let slvs_param = sys.slvs_param(*param_h)?;
                        Ok(slvs_param.val)
                    })
                    .collect();

                Ok(Self {
                    group: Group(slvs_entity.group),
                    workplane: Some(EntityHandle::new(slvs_entity.wrkpl)),
                    coords: T::from(points?),
                })
            }
            Target::In3d => {
                let points: Result<Vec<_>, _> = param_handle_iter
                    .take(3)
                    .map(|param_h| {
                        let slvs_param = sys.slvs_param(*param_h)?;
                        Ok(slvs_param.val)
                    })
                    .collect();

                Ok(Self {
                    group: Group(slvs_entity.group),
                    workplane: Some(EntityHandle::new(slvs_entity.wrkpl)),
                    coords: T::from(points?),
                })
            }
        }
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
