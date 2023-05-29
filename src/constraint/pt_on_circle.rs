use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_CIRCLE},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsArc, EntityHandle, Point},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PtOnCircle<A: AsArc> {
    pub group: Group,
    pub point: EntityHandle<Point>,
    pub arc: EntityHandle<A>,
}

impl<A: AsArc> PtOnCircle<A> {
    pub fn new(group: Group, point: EntityHandle<Point>, arc: EntityHandle<A>) -> Self {
        Self { group, point, arc }
    }
}

impl<A: AsArc> AsGroup for PtOnCircle<A> {
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<A: AsArc> AsSlvsType for PtOnCircle<A> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_CIRCLE as _
    }
}

impl<A: AsArc> AsConstraintData for PtOnCircle<A> {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc.handle(), 0, 0, 0])
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }
}

impl<A: AsArc> FromSystem for PtOnCircle<A> {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_ON_CIRCLE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                arc: EntityHandle::new(slvs_constraint.entityA),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_ON_CIRCLE.")
        }
    }
}
