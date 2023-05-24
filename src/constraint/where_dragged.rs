use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_WHERE_DRAGGED},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
};

define_element!(
    SLVS_C_WHERE_DRAGGED,
    struct WhereDragged {
        point: EntityHandle<Point>,
        workplane: Option<EntityHandle<Workplane>>,
    }
);
impl AsConstraintData for WhereDragged {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         point,
    //         workplane: match slvs_constraint.wrkpl {
    //             0 => None,
    //             h => Some(EntityHandle::new(h)),
    //         },
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}
