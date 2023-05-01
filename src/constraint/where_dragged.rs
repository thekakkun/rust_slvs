use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_WHERE_DRAGGED},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct WhereDragged<P: AsPoint> {
    pub group: Group,
    pub point: Entity<P>,
    pub workplane: Option<Entity<Workplane>>,
}

impl<P: AsPoint> WhereDragged<P> {
    pub fn new(group: Group, point: Entity<P>, workplane: Option<Entity<Workplane>>) -> Self {
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
            point: Entity::new(value.ptA),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
