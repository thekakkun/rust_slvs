use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_FACE_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
    System,
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
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.plane.handle(), 0, 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

impl FromSystem for PtFaceDistance {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_FACE_DISTANCE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                plane: EntityHandle::new(slvs_constraint.entityA),
                distance: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_FACE_DISTANCE.")
        }
    }
}
