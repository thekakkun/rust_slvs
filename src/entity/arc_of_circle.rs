use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Normal, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE},
    element::AsHandle,
    group::Group,
    target::OnWorkplane,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArcOfCircle {
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub center: EntityHandle<Point<OnWorkplane>>,
    pub arc_start: EntityHandle<Point<OnWorkplane>>,
    pub arc_end: EntityHandle<Point<OnWorkplane>>,
    pub normal: EntityHandle<Normal>,
}

impl ArcOfCircle {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        center: EntityHandle<Point<OnWorkplane>>,
        arc_start: EntityHandle<Point<OnWorkplane>>,
        arc_end: EntityHandle<Point<OnWorkplane>>,
        normal: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            workplane,
            center,
            arc_start,
            arc_end,
            normal,
        }
    }
}

impl AsEntityData for ArcOfCircle {
    fn from_system(sys: &System, entity_handle: &EntityHandle<Self>) -> Result<Self, &'static str> {
        let slvs_entity = sys.slvs_entity(entity_handle.handle())?;

        Ok(Self {
            group: Group(slvs_entity.group),
            workplane: EntityHandle::new(slvs_entity.wrkpl),
            center: EntityHandle::new(slvs_entity.point[0]),
            arc_start: EntityHandle::new(slvs_entity.point[1]),
            arc_end: EntityHandle::new(slvs_entity.point[2]),
            normal: EntityHandle::new(slvs_entity.normal),
        })
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
            self.arc_start.handle(),
            self.arc_end.handle(),
        ])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }
}
