use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CUBIC_LINE_TANGENT},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{Cubic, EntityHandle, LineSegment, Workplane},
    group::Group,
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
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let cubic = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
    //     let line = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         workplane: EntityHandle::new(slvs_constraint.wrkpl),
    //         cubic,
    //         line,
    //         to_start: slvs_constraint.other != 0,
    //     })
    // }

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
