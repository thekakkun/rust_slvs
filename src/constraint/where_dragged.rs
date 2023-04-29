use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_WHERE_DRAGGED},
    element::{AsHandle, TypeInfo},
    entity::{AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct WhereDragged<P: AsPoint> {
    point: Entity<P>,
    workplane: Option<Entity<Workplane>>,
}

impl<P: AsPoint> WhereDragged<P> {
    pub fn new(point: Entity<P>, workplane: Option<Entity<Workplane>>) -> Self {
        Self { point, workplane }
    }
}

impl<P: AsPoint> AsConstraintData for WhereDragged<P> {
    fn type_(&self) -> i32 {
        SLVS_C_WHERE_DRAGGED as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
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
            point: Entity::new(value.ptA),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
