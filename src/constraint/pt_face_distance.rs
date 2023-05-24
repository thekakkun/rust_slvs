use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_FACE_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_PT_FACE_DISTANCE,
    struct PtFaceDistance {
        point: EntityHandle<Point>,
        plane: EntityHandle<Workplane>,
        distance: f64,
    }
);

impl AsConstraintData for PtFaceDistance {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         point,
    //         plane: EntityHandle::new(slvs_constraint.entityA),
    //         distance: slvs_constraint.valA,
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}
