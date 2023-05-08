use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_WHERE_DRAGGED},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct WhereDragged<P: AsPoint> {
    pub group: Group,
    pub point: EntityHandle<P>,
    pub workplane: Option<EntityHandle<Workplane>>,
}

impl<P: AsPoint> WhereDragged<P> {
    pub fn new(group: Group, point: EntityHandle<P>, workplane: Option<EntityHandle<Workplane>>) -> Self {
        Self {
            group,
            point,
            workplane,
        }
    }
}

impl<P: AsPoint> AsConstraintData for WhereDragged<P> {
    fn type_(&self) -> i32 {
        SLVS_C_WHERE_DRAGGED as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}

impl<P: AsPoint> TypeInfo for WhereDragged<P> {
    fn type_of() -> String {
        format!("WhereDragged<{}>", P::type_of())
    }
}

impl<P: AsPoint> From<Slvs_Constraint> for WhereDragged<P> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point: EntityHandle::new(value.ptA),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
        }
    }
}
