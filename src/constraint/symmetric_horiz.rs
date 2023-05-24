use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC_HORIZ},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_SYMMETRIC_HORIZ,
    struct SymmetricHoriz {
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    }
);

impl AsConstraintData for SymmetricHoriz {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let point_a = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
    //     let point_b = (*sys.slvs_entity(slvs_constraint.ptB)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         workplane: EntityHandle::new(slvs_constraint.wrkpl),
    //         point_a,
    //         point_b,
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}
