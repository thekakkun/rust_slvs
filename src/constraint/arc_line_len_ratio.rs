use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_LEN_RATIO},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, EntityHandle, LineSegment},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ARC_LINE_LEN_RATIO,
    struct ArcLineLenRatio {
        arc: EntityHandle<ArcOfCircle>,
        line: EntityHandle<LineSegment>,
        ratio: f64,
    }
);

impl AsConstraintData for ArcLineLenRatio {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

impl FromSystem for ArcLineLenRatio {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ARC_LINE_LEN_RATIO == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                arc: EntityHandle::new(slvs_constraint.entityA),
                line: EntityHandle::new(slvs_constraint.entityB),
                ratio: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ARC_LINE_LEN_RATIO.")
        }
    }
}
