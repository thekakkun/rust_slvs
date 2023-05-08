use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_DIAMETER},
    element::{AsHandle, TypeInfo},
    entity::{AsArc, EntityHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

impl<A: AsArc> AsConstraintData for Diameter<A> {
    fn type_(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}

impl<A: AsArc> TypeInfo for Diameter<A> {
    fn type_of() -> String {
        format!("Diameter < {} >", A::type_of())
    }
}

impl<A: AsArc> From<Slvs_Constraint> for Diameter<A> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            arc: EntityHandle::new(value.entityA),
            diameter: value.valA,
        }
    }
}
