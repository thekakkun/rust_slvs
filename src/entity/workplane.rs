use serde::{Deserialize, Serialize};

use super::{AsEntityData, Entity, FromSlvsEntity, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_WORKPLANE},
    element::{AsHandle, TypeInfo},
    group::Group,
    target::{AsTarget, In3d},
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Workplane {
    pub group: Group,
    pub origin: Entity<Point<In3d>>,
    pub normal: Entity<Normal>,
}

impl Workplane {
    pub fn new(group: Group, origin: Entity<Point<In3d>>, normal: Entity<Normal>) -> Self {
        Self {
            group,
            origin,
            normal,
        }
    }
}

impl AsEntityData for Workplane {
    fn type_(&self) -> i32 {
        SLVS_E_WORKPLANE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.origin.handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }
}

impl TypeInfo for Workplane {
    fn type_of() -> String {
        "Workplane".to_string()
    }
}

impl<T: AsTarget> FromSlvsEntity<T> for Workplane {
    fn from(slvs_entity: crate::bindings::Slvs_Entity) -> Self {
        Self {
            group: Group(slvs_entity.group),
            origin: Entity::new(slvs_entity.point[0]),
            normal: Entity::new(slvs_entity.normal),
        }
    }
}
