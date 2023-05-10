use serde::{Deserialize, Serialize};

use super::{
    As2dProjectionTarget, AsEntityData, AsLineSegment, EntityHandle, Point, SomeEntityHandle,
    Workplane,
};
use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_LINE_SEGMENT, SLVS_E_POINT_IN_2D,
        SLVS_E_POINT_IN_3D,
    },
    element::AsHandle,
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
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
    fn into_some_entity_handle(handle: u32) -> SomeEntityHandle {
        match T::slvs_type() as _ {
            SLVS_E_POINT_IN_2D => SomeEntityHandle::LineSegment(LineSegmentHandle::OnWorkplane(
                EntityHandle::new(handle),
            )),
            SLVS_E_POINT_IN_3D => {
                SomeEntityHandle::LineSegment(LineSegmentHandle::In3d(EntityHandle::new(handle)))
            }
            _ => panic!("Unknown slvs_type {}", T::slvs_type()),
        }
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
impl<T: AsTarget> AsLineSegment for LineSegment<T> {}

impl<T: AsTarget> From<Slvs_Entity> for LineSegment<T> {
    fn from(value: Slvs_Entity) -> Self {
        Self {
            group: Group(value.group),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
            point_a: EntityHandle::new(value.point[0]),
            point_b: EntityHandle::new(value.point[1]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineSegmentHandle {
    OnWorkplane(EntityHandle<LineSegment<OnWorkplane>>),
    In3d(EntityHandle<LineSegment<In3d>>),
}

impl AsHandle for LineSegmentHandle {
    fn handle(&self) -> u32 {
        match self {
            Self::OnWorkplane(entity_handle) => entity_handle.handle(),
            Self::In3d(entity_handle) => entity_handle.handle(),
        }
    }
}

impl TryFrom<Slvs_Entity> for LineSegmentHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        if value.type_ == SLVS_E_LINE_SEGMENT as _ {
            match value.wrkpl {
                0 => Ok(LineSegmentHandle::In3d(value.into())),
                _ => Ok(LineSegmentHandle::OnWorkplane(value.into())),
            }
        } else {
            Err("Unexpected Slvs_Entity type")
        }
    }
}
