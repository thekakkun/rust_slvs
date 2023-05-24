use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_ANGLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, LineSegment, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_EQUAL_ANGLE,
    struct EqualAngle {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        line_c: EntityHandle<LineSegment>,
        line_d: EntityHandle<LineSegment>,
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for EqualAngle {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let line_a = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
    //     let line_b = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;
    //     let line_c = (*sys.slvs_entity(slvs_constraint.entityC)?).try_into()?;
    //     let line_d = (*sys.slvs_entity(slvs_constraint.entityD)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         line_a,
    //         line_b,
    //         line_c,
    //         line_d,
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
        Some(vec![
            self.line_a.handle(),
            self.line_b.handle(),
            self.line_c.handle(),
            self.line_d.handle(),
        ])
    }
}
