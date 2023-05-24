use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_PT_LN_DISTANCES},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_EQ_PT_LN_DISTANCES,
    struct EqPtLnDistances {
        line_a: EntityHandle<LineSegment>,
        point_a: EntityHandle<Point>,
        line_b: EntityHandle<LineSegment>,
        point_b: EntityHandle<Point>,
        workplane: Option<EntityHandle<Workplane>>,
    }
);
impl AsConstraintData for EqPtLnDistances {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let line_a = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;
    //     let point_a = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
    //     let line_b = (*sys.slvs_entity(slvs_constraint.entityB)?).try_into()?;
    //     let point_b = (*sys.slvs_entity(slvs_constraint.ptB)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         line_a,
    //         point_a,
    //         line_b,
    //         point_b,
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
        Some(vec![self.line_a.handle(), self.line_b.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}
