use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_IN_PLANE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group, System,
};

define_element!(
    SLVS_C_PT_IN_PLANE,
    struct PtInPlane {
        point: EntityHandle<Point>,
        plane: EntityHandle<Workplane>,
    }
);
impl AsConstraintData for PtInPlane {


    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }
}

impl FromSystem for PtInPlane {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_IN_PLANE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                plane: EntityHandle::new(slvs_constraint.entityA),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_IN_PLANE.")
        }
    }
}
