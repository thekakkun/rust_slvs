use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Normal, Point};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_WORKPLANE},
    element::{AsHandle, TypeInfo},
    group::Group,
    target::In3d,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Workplane {
    pub group: Group,
    pub origin: EntityHandle<Point<In3d>>,
    pub normal: EntityHandle<Normal>,
}

impl Workplane {
    pub fn new(
        group: Group,
        origin: EntityHandle<Point<In3d>>,
        normal: EntityHandle<Normal>,
    ) -> Self {
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

impl From<Slvs_Entity> for Workplane {
    fn from(value: Slvs_Entity) -> Self {
        Self {
            group: Group(value.group),
            origin: EntityHandle::new(value.point[0]),
            normal: EntityHandle::new(value.normal),
        }
    }
}
