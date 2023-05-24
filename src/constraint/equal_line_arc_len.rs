use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_LINE_ARC_LEN},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{ArcOfCircle, EntityHandle, LineSegment, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_EQUAL_LINE_ARC_LEN,
    struct EqualLineArcLen {
        line: EntityHandle<LineSegment>,
        arc: EntityHandle<ArcOfCircle>,
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for EqualLineArcLen {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let line = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         line,
    //         arc: EntityHandle::new(slvs_constraint.entityB),
    //         workplane: match slvs_constraint.wrkpl {
    //             0 => None,
    //             h => Some(EntityHandle::new(h)),
    //         },
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle(), self.arc.handle()])
    }
}
