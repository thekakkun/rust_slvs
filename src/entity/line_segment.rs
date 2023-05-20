use serde::{Deserialize, Serialize};

use super::{
    As2dProjectionTarget, AsEntityData, AsLineSegment, EntityHandle, LineSegmentHandle, Point,
    Workplane,
};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_LINE_SEGMENT},
    element::AsHandle,
    group::Group,
    target::{AsTarget, In3d, OnWorkplane, Target},
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineSegment<T: AsTarget> {
    pub group: Group,
    pub workplane: Option<EntityHandle<Workplane>>,
    pub point_a: EntityHandle<Point<T>>,
    pub point_b: EntityHandle<Point<T>>,
}

impl LineSegment<OnWorkplane> {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point<OnWorkplane>>,
        point_b: EntityHandle<Point<OnWorkplane>>,
    ) -> Self {
        Self {
            group,
            workplane: Some(workplane),
            point_a,
            point_b,
        }
    }
}

impl LineSegment<In3d> {
    pub fn new(
        group: Group,
        point_a: EntityHandle<Point<In3d>>,
        point_b: EntityHandle<Point<In3d>>,
    ) -> Self {
        Self {
            group,
            workplane: None,
            point_a,
            point_b,
        }
    }
}

impl<T: AsTarget> AsEntityData for LineSegment<T> {
    fn from_system(
        sys: &crate::System,
        entity_handle: &EntityHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_entity = sys.slvs_entity(entity_handle.handle())?;

        Ok(Self {
            group: Group(slvs_entity.group),
            workplane: match slvs_entity.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
            point_a: EntityHandle::new(slvs_entity.point[0]),
            point_b: EntityHandle::new(slvs_entity.point[1]),
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_E_LINE_SEGMENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}

impl<T: AsTarget> As2dProjectionTarget for LineSegment<T> {}
impl<T: AsTarget> AsLineSegment for LineSegment<T> {
    fn into_line_segment_handle(handle: u32) -> LineSegmentHandle {
        match T::target_type() {
            Target::OnWorkplane => LineSegmentHandle::OnWorkplane(EntityHandle::new(handle)),
            Target::In3d => LineSegmentHandle::In3d(EntityHandle::new(handle)),
        }
    }
}
