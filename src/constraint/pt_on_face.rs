use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_PT_ON_FACE},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct PtOnFace<P: AsPoint> {
    pub group: Group,
    pub point: Entity<P>,
    pub plane: Entity<Workplane>,
}

impl<P: AsPoint> PtOnFace<P> {
    pub fn new(group: Group, point: Entity<P>, plane: Entity<Workplane>) -> Self {
        Self {
            group,
            point,
            plane,
        }
    }
}

impl<P: AsPoint> AsConstraintData for PtOnFace<P> {
    fn type_(&self) -> i32 {
        SLVS_C_PT_ON_FACE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> u32 {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }
}

impl<P: AsPoint> TypeInfo for PtOnFace<P> {
    fn type_of() -> String {
        format!("PtOnFace < {} >", P::type_of())
    }
}

impl<P: AsPoint> From<Slvs_Constraint> for PtOnFace<P> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            point: Entity::new(value.ptA),
            plane: Entity::new(value.entityA),
        }
    }
}
