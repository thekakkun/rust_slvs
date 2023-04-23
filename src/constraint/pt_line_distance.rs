use crate::{
    bindings::{Slvs_hEntity, SLVS_C_PT_LINE_DISTANCE},
    element::{AsHandle, Target},
    entity::{Entity, LineSegment, Point, SomeLineSegment, SomePoint},
};

use super::AsConstraintData;

pub struct PtLineDistance {
    point: Entity<SomePoint>,
    line: Entity<SomeLineSegment>,
    distance: f64,
}

// impl PtLineDistance {
//     pub fn new(
//         point: Entity<Point<dyn Target>>,
//         line: Entity<LineSegment<dyn Target>>,
//         distance: f64,
//     ) -> Self {
//         Self {
//             point,
//             line,
//             distance,
//         }
//     }
// }

// impl AsConstraintData for PtLineDistance {
//     type Apply = dyn Target;

//     fn type_(&self) -> i32 {
//         SLVS_C_PT_LINE_DISTANCE as _
//     }

//     fn val(&self) -> Option<f64> {
//         Some(self.distance)
//     }

//     fn points(&self) -> Option<Vec<Slvs_hEntity>> {
//         Some(vec![self.point.as_handle(), self.line.as_handle()])
//     }

//     fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
//         None
//     }

//     fn others(&self) -> [bool; 2] {
//         [false, false]
//     }
// }
