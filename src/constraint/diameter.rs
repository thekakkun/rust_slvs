use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_DIAMETER},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsArc, EntityHandle},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diameter<A: AsArc> {
    pub group: Group,
    pub arc: EntityHandle<A>,
    pub diameter: f64,
}

impl<A: AsArc> Diameter<A> {
    pub fn new(group: Group, arc: EntityHandle<A>, diameter: f64) -> Self {
        Self {
            group,
            arc,
            diameter,
        }
    }
}

impl<A: AsArc> AsGroup for Diameter<A> {
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<A: AsArc> AsSlvsType for Diameter<A> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }
}

impl<A: AsArc> AsConstraintData for Diameter<A> {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc.handle(), 0, 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}

impl<A: AsArc> FromSystem for Diameter<A> {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_DIAMETER == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                arc: EntityHandle::new(slvs_constraint.entityA),
                diameter: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_DIAMETER.")
        }
    }
}
