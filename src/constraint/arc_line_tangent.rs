use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_ARC_LINE_TANGENT},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, AsLineSegment, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct ArcLineTangent<L: AsLineSegment> {
    workplane: Entity<Workplane>,
    arc: Entity<ArcOfCircle>,
    line: Entity<L>,
    to_beginning: bool,
}

impl<L: AsLineSegment> ArcLineTangent<L> {
    pub fn new(
        workplane: Entity<Workplane>,
        arc: Entity<ArcOfCircle>,
        line: Entity<L>,
        to_beginning: bool,
    ) -> Self {
        Self {
            workplane,
            arc,
            line,
            to_beginning,
        }
    }
}

impl<L: AsLineSegment> AsConstraintData for ArcLineTangent<L> {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_LINE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_beginning, false]
    }
}

impl<L: AsLineSegment> TypeInfo for ArcLineTangent<L> {
    fn type_of() -> String {
        format!("ArcLineTangent<{}>", L::type_of())
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for ArcLineTangent<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            workplane: Entity::new(value.wrkpl),
            arc: Entity::new(value.entityA),
            line: Entity::new(value.entityB),
            to_beginning: value.other != 0,
        }
    }
}
