use std::rc::Weak;

use crate::bindings;

use super::{group::Group, param::Param, Handle};

enum EntityType {
    Undefined,
    PointIn3d = bindings::SLVS_E_POINT_IN_3D as isize,
    PointIn2d = bindings::SLVS_E_POINT_IN_2D as isize,
    NormalIn3d = bindings::SLVS_E_NORMAL_IN_3D as isize,
    NormalIn2d = bindings::SLVS_E_NORMAL_IN_2D as isize,
    Distance = bindings::SLVS_E_DISTANCE as isize,
    Workplane = bindings::SLVS_E_WORKPLANE as isize,
    LineSegment = bindings::SLVS_E_LINE_SEGMENT as isize,
    Cubic = bindings::SLVS_E_CUBIC as isize,
    Circle = bindings::SLVS_E_CIRCLE as isize,
    ArcOfCircle = bindings::SLVS_E_ARC_OF_CIRCLE as isize,
}

pub struct Entity {
    h: u32,
    group: Weak<Group>,
    type_: EntityType,
    wrkpl: Option<Weak<Entity>>,
    point: [Option<Weak<Entity>>; 4],
    normal: Option<Weak<Entity>>,
    distance: Option<Weak<Entity>>,
    param: [Option<Weak<Param>>; 4],
}

impl Handle for Entity {
    fn get_handle(&self) -> u32 {
        self.h
    }

    fn set_handle(&mut self, h: u32) {
        self.h = h;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            h: 0,
            group: Weak::new(),
            type_: EntityType::Undefined,
            wrkpl: None,
            point: [None, None, None, None],
            normal: None,
            distance: None,
            param: [None, None, None, None],
        }
    }
}

impl Entity {
    pub fn new_point_3d(
        group: &Weak<Group>,
        x: &Weak<Param>,
        y: &Weak<Param>,
        z: &Weak<Param>,
    ) -> Self {
        Entity {
            group: Weak::clone(group),
            type_: EntityType::PointIn3d,
            param: [
                Some(Weak::clone(x)),
                Some(Weak::clone(y)),
                Some(Weak::clone(z)),
                None,
            ],
            ..Default::default()
        }
    }
}

// impl Elements<Entity> {
//     pub fn add_point_3d(
//         &mut self,
//         group: &Weak<Group>,
//         x: &Weak<Param>,
//         y: &Weak<Param>,
//         z: &Weak<Param>,
//     ) -> Weak<Entity> {
//         let new_point_3d = Rc::new(Entity {
//             h: self.h_gen.next().unwrap(),
//             group: Weak::clone(group),
//             type_: EntityType::PointIn2d,
//             param: [
//                 Some(Weak::clone(x)),
//                 Some(Weak::clone(y)),
//                 Some(Weak::clone(z)),
//                 None,
//             ],
//             ..Default::default()
//         });

//         self.list.push(Rc::clone(&new_point_3d));
//         Rc::downgrade(&new_point_3d)
//     }

//     pub fn add_point_2d(
//         &mut self,
//         group: &Weak<Group>,
//         x: &Weak<Param>,
//         y: &Weak<Param>,
//         z: &Weak<Param>,
//     ) -> Weak<Entity> {
//         let new_point_2d = Rc::new(Entity {
//             h: self.h_gen.next().unwrap(),
//             group: Weak::clone(group),
//             type_: EntityType::PointIn2d,
//             param: [
//                 Some(Weak::clone(x)),
//                 Some(Weak::clone(y)),
//                 Some(Weak::clone(z)),
//                 None,
//             ],
//             ..Default::default()
//         });

//         self.list.push(Rc::clone(&new_point_2d));
//         Rc::downgrade(&new_point_2d)
//     }
// }
