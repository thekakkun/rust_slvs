use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_PT_LN_DISTANCES},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
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

impl FromSystem for EqPtLnDistances {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQ_PT_LN_DISTANCES == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_EQ_PT_LN_DISTANCES.")
        }
    }
}
