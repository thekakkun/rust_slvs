use super::{AsArc, AsEntityData, Entity, Normal, Point};
use crate::{
    bindings::{Slvs_hEntity, SLVS_E_ARC_OF_CIRCLE},
    element::{AsHandle, OnWorkplane},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcOfCircle {
    pub center: Entity<Point<OnWorkplane>>,
    pub arc_begin: Entity<Point<OnWorkplane>>,
    pub arc_end: Entity<Point<OnWorkplane>>,
    pub normal: Entity<Normal<OnWorkplane>>,
}

impl ArcOfCircle {
    pub fn new(
        center: Entity<Point<OnWorkplane>>,
        arc_begin: Entity<Point<OnWorkplane>>,
        arc_end: Entity<Point<OnWorkplane>>,
        normal: Entity<Normal<OnWorkplane>>,
    ) -> Self {
        Self {
            center,
            arc_begin,
            arc_end,
            normal,
        }
    }
}

impl AsArc for ArcOfCircle {}

impl AsEntityData for ArcOfCircle {
    type Sketch = OnWorkplane;

    fn type_(&self) -> i32 {
        SLVS_E_ARC_OF_CIRCLE as _
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.arc_begin.as_handle(),
            self.arc_begin.as_handle(),
            self.arc_end.as_handle(),
        ])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.as_handle())
    }
}
