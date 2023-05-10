use serde::{Deserialize, Serialize};

use super::{
    AsArc, AsCurve, AsEntityData, EntityHandle, Normal, Point, SomeEntityHandle, Workplane,
};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE},
    element::AsHandle,
    group::Group,
    target::OnWorkplane,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcOfCircle {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub center: EntityHandle<Point<OnWorkplane>>,
    pub arc_begin: EntityHandle<Point<OnWorkplane>>,
    pub arc_end: EntityHandle<Point<OnWorkplane>>,
    pub normal: EntityHandle<Normal>,
}

impl ArcOfCircle {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        center: EntityHandle<Point<OnWorkplane>>,
        arc_begin: EntityHandle<Point<OnWorkplane>>,
        arc_end: EntityHandle<Point<OnWorkplane>>,
        normal: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            workplane,
            center,
            arc_begin,
            arc_end,
            normal,
        }
    }
}

impl AsArc for ArcOfCircle {}
impl AsCurve for ArcOfCircle {}

impl AsEntityData for ArcOfCircle {
    fn into_some_entity_handle(handle: u32) -> SomeEntityHandle {
        SomeEntityHandle::ArcOfCircle(EntityHandle::new(handle))
    }

    fn slvs_type(&self) -> i32 {
        SLVS_E_ARC_OF_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.center.handle(),
            self.arc_begin.handle(),
            self.arc_end.handle(),
        ])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }
}

impl From<Slvs_Entity> for ArcOfCircle {
    fn from(value: Slvs_Entity) -> Self {
        Self {
            group: Group(value.group),
            workplane: EntityHandle::new(value.wrkpl),
            center: EntityHandle::new(value.point[0]),
            arc_begin: EntityHandle::new(value.point[1]),
            arc_end: EntityHandle::new(value.point[2]),
            normal: EntityHandle::new(value.normal),
        }
    }
}
