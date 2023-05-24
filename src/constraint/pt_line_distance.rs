use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_LINE_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_PT_LINE_DISTANCE,
    struct PtLineDistance {
        point: EntityHandle<Point>,
        line: EntityHandle<LineSegment>,
        distance: f64,
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for PtLineDistance {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
    //     let line = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         point,
    //         line,
    //         distance: slvs_constraint.valA,
    //         workplane: match slvs_constraint.wrkpl {
    //             0 => None,
    //             h => Some(EntityHandle::new(h)),
    //         },
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}
