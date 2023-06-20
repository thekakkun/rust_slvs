use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::{AsEntityData, Distance, EntityHandle, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_CIRCLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

define_element!(
    SLVS_E_CIRCLE,
    /// A complete circle.
    ///
    /// The orientation is defined by a [Normal][crate::entity::Normal], and the size
    /// of the circle is defined by passing a handle to a [Distance][crate::entity::Distance]
    /// entity.
    ///
    /// See the [module-level documentation][crate] for usage example.
    struct Circle {
        /// The circle lies within a plane with this normal
        normal: EntityHandle<Normal>,
        center: EntityHandle<Point>,
        radius: EntityHandle<Distance>,
    }
);

impl AsEntityData for Circle {
    fn points(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.center.handle(), 0, 0, 0])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        Some(self.radius.handle())
    }
}

impl FromSystem for Circle {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;

        if SLVS_E_CIRCLE == slvs_entity.type_ as _ {
            Ok(Self {
                group: Group(slvs_entity.group),
                normal: EntityHandle::new(slvs_entity.normal),
                center: EntityHandle::new(slvs_entity.point[0]),
                radius: EntityHandle::new(slvs_entity.distance),
            })
        } else {
            Err("Expected entity to have type SLVS_E_CIRCLE.")
        }
    }
}
