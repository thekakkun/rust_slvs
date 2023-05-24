use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_CIRCLE},
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{AsRadiused, EntityHandle, Point},
    group::Group,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PtOnCircle<R: AsRadiused> {
    pub group: Group,
    pub point: EntityHandle<Point>,
    pub circle: EntityHandle<R>,
}

impl<R: AsRadiused> PtOnCircle<R> {
    fn new(group: Group, point: EntityHandle<Point>, circle: EntityHandle<R>) -> Self {
        Self {
            group,
            point,
            circle,
        }
    }
}

impl<R: AsRadiused> AsGroup for PtOnCircle<R> {
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<R: AsRadiused> AsSlvsType for PtOnCircle<R> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_CIRCLE as _
    }
}

impl<R: AsRadiused> AsConstraintData for PtOnCircle<R> {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
    //     let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;
    //     let arc = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into()?;

    //     Ok(Self {
    //         group: Group(slvs_constraint.group),
    //         point,
    //         circle: arc,
    //     })
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.circle.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}
