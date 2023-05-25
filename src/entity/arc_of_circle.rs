use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

// /// A circular arc.
// ///
// /// An arc must always lie within a workplane; it cannot be free in 3d.
// /// So it is specified with a workplane.
// ///
// /// An extra constraint is generated automatically to ensure that
// /// `distance(center, beginning) = distance(center, end)`.
// ///
// /// See the [module-level documentation][crate] for usage examples.
define_element!(
    SLVS_E_ARC_OF_CIRCLE,
    struct ArcOfCircle {
        workplane: EntityHandle<Workplane>,
        center: EntityHandle<Point>,
        arc_start: EntityHandle<Point>,
        arc_end: EntityHandle<Point>,
    }
);

impl AsEntityData for ArcOfCircle {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([
            self.center.handle(),
            self.arc_start.handle(),
            self.arc_end.handle(),
            0,
        ])
    }
}

impl FromSystem for ArcOfCircle {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;

        if SLVS_E_ARC_OF_CIRCLE == slvs_entity.type_ as _ {
            Ok(Self {
                group: Group(slvs_entity.group),
                workplane: EntityHandle::new(slvs_entity.wrkpl),
                center: EntityHandle::new(slvs_entity.point[0]),
                arc_start: EntityHandle::new(slvs_entity.point[1]),
                arc_end: EntityHandle::new(slvs_entity.point[2]),
            })
        } else {
            Err("Expected entity to have type SLVS_E_ARC_OF_CIRCLE.")
        }
    }
}
