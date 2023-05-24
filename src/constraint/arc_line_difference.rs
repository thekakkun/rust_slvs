use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_DIFFERENCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{ArcOfCircle, EntityHandle, LineSegment},
    group::Group,
};

define_element!(
    SLVS_C_ARC_LINE_DIFFERENCE,
    struct ArcLineDifference {
        arc: EntityHandle<ArcOfCircle>,
        line: EntityHandle<LineSegment>,
        difference: f64,
    }
);

impl AsConstraintData for ArcLineDifference {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let line = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         arc: EntityHandle::new(slvs_constraint.entityA),
    //         line,
    //         difference: slvs_constraint.valA,
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}
