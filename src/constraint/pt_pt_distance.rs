use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_PT_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_PT_PT_DISTANCE,
    struct PtPtDistance {
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        distance: f64,
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for PtPtDistance {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let point_a = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
    //     let point_b = (*sys.slvs_entity(slvs_constraint.ptB)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         point_a,
    //         point_b,
    //         distance: slvs_constraint.valA,
    //         workplane: match slvs_constraint.wrkpl {
    //             0 => None,
    //             h => Some(EntityHandle::new(h)),
    //         },
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }
}
