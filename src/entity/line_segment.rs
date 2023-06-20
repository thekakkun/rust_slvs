use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Point};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_LINE_SEGMENT},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

define_element!(
    SLVS_E_LINE_SEGMENT,
    /// A line segment between two points.
    ///
    /// See the [module-level documentation][crate] for usage example.
    struct LineSegment {
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    }
);

impl AsEntityData for LineSegment {
    fn points(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.point_a.handle(), self.point_b.handle(), 0, 0])
    }
}

impl FromSystem for LineSegment {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;

        if SLVS_E_LINE_SEGMENT == slvs_entity.type_ as _ {
            Ok(Self {
                group: Group(slvs_entity.group),
                point_a: EntityHandle::new(slvs_entity.point[0]),
                point_b: EntityHandle::new(slvs_entity.point[1]),
            })
        } else {
            Err("Expected entity to have type SLVS_E_LINE_SEGMENT.")
        }
    }
}
