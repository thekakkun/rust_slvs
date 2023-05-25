use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_WORKPLANE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
};

define_element!(
    SLVS_E_WORKPLANE,
    struct Workplane {
        origin: EntityHandle<Point>,
        normal: EntityHandle<Normal>,
    }
);

impl AsEntityData for Workplane {
    fn points(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.origin.handle(), 0, 0, 0])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }
}

impl FromSystem for Workplane {
    fn from_system(sys: &crate::System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;

        if SLVS_E_WORKPLANE == slvs_entity.type_ as _ {
            Ok(Self {
                group: Group(slvs_entity.group),
                origin: EntityHandle::new(slvs_entity.point[0]),
                normal: EntityHandle::new(slvs_entity.normal),
            })
        } else {
            Err("Expected entity to have type SLVS_E_WORKPLANE")
        }
    }
}
