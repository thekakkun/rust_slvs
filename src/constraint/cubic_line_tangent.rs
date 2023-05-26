use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CUBIC_LINE_TANGENT},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{Cubic, EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_CUBIC_LINE_TANGENT,
    struct CubicLineTangent {
        workplane: EntityHandle<Workplane>,
        cubic: EntityHandle<Cubic>,
        line: EntityHandle<LineSegment>,
        to_start: bool,
    }
);

impl AsConstraintData for CubicLineTangent {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.cubic.handle(), self.line.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_start, false]
    }
}

impl FromSystem for CubicLineTangent {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_CUBIC_LINE_TANGENT == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                workplane: EntityHandle::new(slvs_constraint.wrkpl),
                cubic: EntityHandle::new(slvs_constraint.entityA),
                line: EntityHandle::new(slvs_constraint.entityB),
                to_start: slvs_constraint.other != 0,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_CUBIC_LINE_TANGENT.")
        }
    }
}
