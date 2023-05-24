use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SAME_ORIENTATION},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, Normal},
    group::Group,
};

define_element!(
    SLVS_C_SAME_ORIENTATION,
    struct SameOrientation {
        normal_a: EntityHandle<Normal>,
        normal_b: EntityHandle<Normal>,
    }
);

impl AsConstraintData for SameOrientation {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         normal_a: EntityHandle::new(slvs_constraint.entityA),
    //         normal_b: EntityHandle::new(slvs_constraint.entityB),
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.normal_a.handle(), self.normal_b.handle()])
    }
}
