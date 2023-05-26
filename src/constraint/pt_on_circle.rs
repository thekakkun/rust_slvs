use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_CIRCLE},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsRadiused, EntityHandle, Point},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PtOnCircle<R: AsRadiused> {
    pub group: Group,
    pub point: EntityHandle<Point>,
    pub circle: EntityHandle<R>,
}

impl<R: AsRadiused> PtOnCircle<R> {
    pub fn new(group: Group, point: EntityHandle<Point>, circle: EntityHandle<R>) -> Self {
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

impl<R: AsRadiused> FromSystem for PtOnCircle<R> {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_ON_CIRCLE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                circle: EntityHandle::new(slvs_constraint.entityA),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_ON_CIRCLE.")
        }
    }
}
