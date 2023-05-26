use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_VERTICAL},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Vertical {
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

impl Vertical {
    pub fn new_points(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    ) -> Self {
        Vertical::Points {
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
        Vertical::Line {
            group,
            workplane,
            line,
        }
    }
}

impl AsGroup for Vertical {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Vertical::Points { group, .. } => group.handle(),
            Vertical::Line { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Vertical {
    fn slvs_type(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }
}

impl AsConstraintData for Vertical {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Vertical::Points { workplane, .. } => Some(workplane.handle()),
            Vertical::Line { workplane, .. } => Some(workplane.handle()),
        }
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        match self {
            Vertical::Points {
                point_a, point_b, ..
            } => Some(vec![point_a.handle(), point_b.handle()]),
            Vertical::Line { .. } => None,
        }
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        match self {
            Vertical::Points { .. } => None,
            Vertical::Line { line, .. } => Some(vec![line.handle()]),
        }
    }
}

impl FromSystem for Vertical {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_VERTICAL == slvs_constraint.type_ as _ {
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
                Err("Vertical should have handle for line or two points.")
            }
        } else {
            Err("Expected constraint to have type SLVS_C_VERTICAL.")
        }
    }
}
