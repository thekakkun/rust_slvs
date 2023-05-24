use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_ARC_LEN_RATIO},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{ArcOfCircle, EntityHandle},
    group::Group,
};

define_element!(
    SLVS_C_ARC_ARC_LEN_RATIO,
    struct ArcArcLenRatio {
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
        ratio: f64,
    }
);

impl AsConstraintData for ArcArcLenRatio {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         arc_a: EntityHandle::new(slvs_constraint.entityA),
    //         arc_b: EntityHandle::new(slvs_constraint.entityB),
    //         ratio: slvs_constraint.valA,
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.handle(), self.arc_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}
