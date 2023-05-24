use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_LINE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_PT_ON_LINE,
    struct PtOnLine {
        point: EntityHandle<Point>,
        line: EntityHandle<LineSegment>,
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for PtOnLine {
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
    //         workplane: match slvs_constraint.wrkpl {
    //             0 => None,
    //             h => Some(EntityHandle::new(h)),
    //         },
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }
}
