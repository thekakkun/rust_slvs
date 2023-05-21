use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_HORIZONTAL},
    element::AsHandle,
    entity::{EntityHandle, LineSegmentHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Horizontal {
    Points {
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: PointHandle,
        point_b: PointHandle,
    },
    Line {
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: LineSegmentHandle,
    },
}

impl Horizontal {
    pub fn new_points(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: PointHandle,
        point_b: PointHandle,
    ) -> Self {
        Horizontal::Points {
            group,
            workplane,
            point_a,
            point_b,
        }
    }

    pub fn new_line(
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: LineSegmentHandle,
    ) -> Self {
        Horizontal::Line {
            group,
            workplane,
            line,
        }
    }
}

impl AsConstraintData for Horizontal {
    fn slvs_type(&self) -> i32 {
        SLVS_C_HORIZONTAL as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Horizontal::Points { workplane, .. } => Some(workplane.handle()),
            Horizontal::Line { workplane, .. } => Some(workplane.handle()),
        }
    }

    fn group(&self) -> Slvs_hGroup {
        match self {
            Horizontal::Points { group, .. } => group.handle(),
            Horizontal::Line { group, .. } => group.handle(),
        }
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        match self {
            Horizontal::Points {
                point_a, point_b, ..
            } => Some(vec![point_a.handle(), point_b.handle()]),
            Horizontal::Line { .. } => None,
        }
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        match self {
            Horizontal::Points { .. } => None,
            Horizontal::Line { line, .. } => Some(vec![line.handle()]),
        }
    }
}
