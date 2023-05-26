use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PROJ_PT_DISTANCE},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsProjectionTarget, EntityHandle, Point},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjPtDistance<L: AsProjectionTarget> {
    pub group: Group,
    pub point_a: EntityHandle<Point>,
    pub point_b: EntityHandle<Point>,
    pub on_line: EntityHandle<L>,
    pub distance: f64,
}

impl<L: AsProjectionTarget> ProjPtDistance<L> {
    pub fn new(
        group: Group,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        on_line: EntityHandle<L>,
        distance: f64,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            on_line,
            distance,
        }
    }
}
impl<L: AsProjectionTarget> AsGroup for ProjPtDistance<L> {
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<L: AsProjectionTarget> AsSlvsType for ProjPtDistance<L> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PROJ_PT_DISTANCE as _
    }
}

impl<L: AsProjectionTarget> AsConstraintData for ProjPtDistance<L> {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.on_line.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point_a.handle(), self.point_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

impl<L: AsProjectionTarget> FromSystem for ProjPtDistance<L> {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PROJ_PT_DISTANCE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                on_line: EntityHandle::new(slvs_constraint.entityA),
                distance: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PROJ_PT_DISTANCE.")
        }
    }
}
