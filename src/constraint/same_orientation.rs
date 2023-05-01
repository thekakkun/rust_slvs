use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_SAME_ORIENTATION},
    element::{AsHandle, TypeInfo},
    entity::{Entity, Normal},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct SameOrientation {
    pub group: Group,
    pub normal_a: Entity<Normal>,
    pub normal_b: Entity<Normal>,
}

impl SameOrientation {
    pub fn new(group: Group, normal_a: Entity<Normal>, normal_b: Entity<Normal>) -> Self {
        Self {
            group,
            normal_a,
            normal_b,
        }
    }
}

impl AsConstraintData for SameOrientation {
    fn type_(&self) -> i32 {
        SLVS_C_SAME_ORIENTATION as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> u32 {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.normal_a.handle(), self.normal_b.handle()])
    }
}

impl TypeInfo for SameOrientation {
    fn type_of() -> String {
        "SameOrientation".to_string()
    }
}

impl From<Slvs_Constraint> for SameOrientation {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            normal_a: Entity::new(value.entityA),
            normal_b: Entity::new(value.entityB),
        }
    }
}
