use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_HORIZONTAL},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Horizontal {
    Points {
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    },
    Line {
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: EntityHandle<LineSegment>,
    },
}

impl Horizontal {
    pub fn new_points(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
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
        line: EntityHandle<LineSegment>,
    ) -> Self {
        Horizontal::Line {
            group,
            workplane,
            line,
        }
    }
}

impl AsGroup for Horizontal {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Horizontal::Points { group, .. } => group.handle(),
            Horizontal::Line { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Horizontal {
    fn slvs_type(&self) -> i32 {
        SLVS_C_HORIZONTAL as _
    }
}

impl AsConstraintData for Horizontal {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Horizontal::Points { workplane, .. } => Some(workplane.handle()),
            Horizontal::Line { workplane, .. } => Some(workplane.handle()),
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

impl FromSystem for Horizontal {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_HORIZONTAL == slvs_constraint.type_ as _ {
            if slvs_constraint.ptA != 0 && slvs_constraint.ptB != 0 {
                Ok(Self::Points {
                    group: Group(slvs_constraint.group),
                    workplane: EntityHandle::new(slvs_constraint.wrkpl),
                    point_a: EntityHandle::new(slvs_constraint.ptA),
                    point_b: EntityHandle::new(slvs_constraint.ptB),
                })
            } else if slvs_constraint.entityA != 0 {
                Ok(Self::Line {
                    group: Group(slvs_constraint.group),
                    workplane: EntityHandle::new(slvs_constraint.wrkpl),
                    line: EntityHandle::new(slvs_constraint.entityA),
                })
            } else {
                Err("Horizontal should have handle for line or two points.")
            }
        } else {
            Err("Expected constraint to have type SLVS_C_HORIZONTAL.")
        }
    }
}
