use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Point};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_CUBIC},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    group::Group,
};

define_element!(
    SLVS_E_CUBIC,
    struct Cubic {
        start_point: EntityHandle<Point>,
        start_control: EntityHandle<Point>,
        end_control: EntityHandle<Point>,
        end_point: EntityHandle<Point>,
    }
);

impl AsEntityData for Cubic {
    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.start_point.handle(),
            self.start_control.handle(),
            self.end_control.handle(),
            self.end_point.handle(),
        ])
    }
}

// /// A non-rational cubic Bezier segment.
// #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
// pub struct Cubic<T: AsTarget> {
//     pub group: Group,
//     pub workplane: Option<EntityHandle<Workplane>>,
//     pub start_point: EntityHandle<Point<T>>,
//     pub start_control: EntityHandle<Point<T>>,
//     pub end_control: EntityHandle<Point<T>>,
//     pub end_point: EntityHandle<Point<T>>,
// }

// impl Cubic<OnWorkplane> {
//     /// Constructs a new `Cubic` on a workplane.
//     pub fn new(
//         group: Group,
//         workplane: EntityHandle<Workplane>,
//         start_point: EntityHandle<Point<OnWorkplane>>,
//         start_control: EntityHandle<Point<OnWorkplane>>,
//         end_control: EntityHandle<Point<OnWorkplane>>,
//         end_point: EntityHandle<Point<OnWorkplane>>,
//     ) -> Self {
//         Self {
//             group,
//             workplane: Some(workplane),
//             start_point,
//             start_control,
//             end_control,
//             end_point,
//         }
//     }
// }

// impl Cubic<In3d> {
//     /// Constructs a new `Cubic` in 3d space.
//     pub fn new(
//         group: Group,
//         start_point: EntityHandle<Point<In3d>>,
//         start_control: EntityHandle<Point<In3d>>,
//         end_control: EntityHandle<Point<In3d>>,
//         end_point: EntityHandle<Point<In3d>>,
//     ) -> Self {
//         Self {
//             group,
//             workplane: None,
//             start_point,
//             start_control,
//             end_control,
//             end_point,
//         }
//     }
// }

// impl<T: AsTarget> AsEntityData for Cubic<T> {
//     fn from_system(sys: &System, entity_handle: &EntityHandle<Self>) -> Result<Self, &'static str> {
//         let slvs_entity = sys.slvs_entity(entity_handle.handle())?;

//         Ok(Self {
//             group: Group(slvs_entity.group),
//             workplane: match slvs_entity.wrkpl {
//                 0 => None,
//                 h => Some(EntityHandle::new(h)),
//             },
//             start_point: EntityHandle::new(slvs_entity.point[0]),
//             start_control: EntityHandle::new(slvs_entity.point[1]),
//             end_control: EntityHandle::new(slvs_entity.point[2]),
//             end_point: EntityHandle::new(slvs_entity.point[3]),
//         })
//     }

//     fn slvs_type(&self) -> i32 {
//         SLVS_E_CUBIC as _
//     }

//     fn workplane(&self) -> Option<Slvs_hEntity> {
//         self.workplane.map(|workplane| workplane.handle())
//     }

//     fn group(&self) -> Slvs_hGroup {
//         self.group.handle()
//     }

//     fn points(&self) -> Option<Vec<Slvs_hEntity>> {
//         Some(vec![
//             self.start_point.handle(),
//             self.start_control.handle(),
//             self.end_control.handle(),
//             self.end_point.handle(),
//         ])
//     }
// }
