use super::{AsEntityData, AsPoint, Entity, FromSlvsEntity, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_POINT_IN_2D},
    element::{AsHandle, TypeInfo},
    target::{AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T: AsTarget> {
    pub workplane: Option<Entity<Workplane>>,
    pub coords: T,
}

impl Point<OnWorkplane> {
    pub fn new(workplane: Entity<Workplane>, u: f64, v: f64) -> Self {
        Self {
            workplane: Some(workplane),
            coords: OnWorkplane(u, v),
        }
    }
}

impl Point<In3d> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            workplane: None,
            coords: In3d(x, y, z),
        }
    }
}

impl<T: AsTarget> AsPoint for Point<T> {}

impl<T: AsTarget> AsEntityData for Point<T> {
    fn type_(&self) -> i32 {
        <T as AsTarget>::type_()
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|w| w.handle())
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(self.coords.as_vec())
    }
}

impl<T: AsTarget> TypeInfo for Point<T> {
    fn type_of() -> String {
        format!("Point<{}>", T::type_of())
    }
}

impl<T: AsTarget + From<Vec<f64>> + Default> FromSlvsEntity<T> for Point<T> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        if slvs_entity.type_ == SLVS_E_POINT_IN_2D as _ {
            Self {
                workplane: Some(Entity::new(slvs_entity.wrkpl)),
                coords: T::default(),
            }
        } else {
            Self {
                workplane: None,
                coords: T::default(),
            }
        }
    }

    fn set_vals(&mut self, params: Vec<f64>) {
        self.coords = params.into();
    }
}
