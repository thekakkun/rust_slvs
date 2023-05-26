use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_ARC_DIFFERENCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, EntityHandle},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ARC_ARC_DIFFERENCE,
    struct ArcArcDifference {
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
        difference: f64,
    }
);

impl AsConstraintData for ArcArcDifference {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc_a.handle(), self.arc_b.handle(), 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}

impl FromSystem for ArcArcDifference {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ARC_ARC_DIFFERENCE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                arc_a: EntityHandle::new(slvs_constraint.entityA),
                arc_b: EntityHandle::new(slvs_constraint.entityB),
                difference: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ARC_ARC_DIFFERENCE.")
        }
    }
}
