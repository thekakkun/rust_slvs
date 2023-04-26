use super::{AsEntityData, Entity, FromSlvsEntity, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_WORKPLANE},
    element::{AsHandle, AsTarget, In3d},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Workplane {
    pub origin: Entity<Point<In3d>>,
    pub normal: Entity<Normal>,
}

impl Workplane {
    pub fn new(origin: Entity<Point<In3d>>, normal: Entity<Normal>) -> Self {
        Self { origin, normal }
    }
}

impl AsEntityData for Workplane {
    fn type_(&self) -> i32 {
        SLVS_E_WORKPLANE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.origin.as_handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }
}

impl<T: AsTarget> FromSlvsEntity<T> for Workplane {
    fn from(slvs_entity: crate::bindings::Slvs_Entity) -> Self {
        Self {
            origin: Entity::new(slvs_entity.point[0]),
            normal: Entity::new(slvs_entity.normal),
        }
    }
}
