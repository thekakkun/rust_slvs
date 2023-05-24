use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    group::Group,
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

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.center.handle(),
            self.arc_start.handle(),
            self.arc_end.handle(),
        ])
    }
}

// impl ArcOfCircle {
//     /// Constructs a new `ArcOfCircle`.
//     pub fn new(
//         group: Group,
//         workplane: EntityHandle<Workplane>,
//         center: EntityHandle<Point<OnWorkplane>>,
//         arc_start: EntityHandle<Point<OnWorkplane>>,
//         arc_end: EntityHandle<Point<OnWorkplane>>,
//         normal: EntityHandle<Normal>,
//     ) -> Self {
//         Self {
//             group,
//             workplane,
//             center,
//             arc_start,
//             arc_end,
//             normal,
//         }
//     }
// }

// impl AsEntityData for ArcOfCircle {
//     fn from_system(sys: &System, entity_handle: &EntityHandle<Self>) -> Result<Self, &'static str> {
//         let slvs_entity = sys.slvs_entity(entity_handle.handle())?;

//         Ok(Self {
//             group: Group(slvs_entity.group),
//             workplane: EntityHandle::new(slvs_entity.wrkpl),
//             center: EntityHandle::new(slvs_entity.point[0]),
//             arc_start: EntityHandle::new(slvs_entity.point[1]),
//             arc_end: EntityHandle::new(slvs_entity.point[2]),
//             normal: EntityHandle::new(slvs_entity.normal),
//         })
//     }

//     fn slvs_type(&self) -> i32 {
//         SLVS_E_ARC_OF_CIRCLE as _
//     }

//     fn workplane(&self) -> Option<Slvs_hEntity> {
//         Some(self.workplane.handle())
//     }

//     fn group(&self) -> Slvs_hGroup {
//         self.group.handle()
//     }

//     fn points(&self) -> Option<Vec<Slvs_hEntity>> {
//         Some(vec![
//             self.center.handle(),
//             self.arc_start.handle(),
//             self.arc_end.handle(),
//         ])
//     }

//     fn normal(&self) -> Option<Slvs_hEntity> {
//         Some(self.normal.handle())
//     }
// }
