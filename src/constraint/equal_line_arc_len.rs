use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_LINE_ARC_LEN},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, AsLineSegment, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug)]
pub struct EqualLineArcLen<L: AsLineSegment> {
    pub group: Group,
    pub line: Entity<L>,
    pub arc: Entity<ArcOfCircle>,
    pub workplane: Option<Entity<Workplane>>,
}

impl<L: AsLineSegment> EqualLineArcLen<L> {
    pub fn new(
        group: Group,
        line: Entity<L>,
        arc: Entity<ArcOfCircle>,
        workplane: Option<Entity<Workplane>>,
    ) -> Self {
        Self {
            group,
            line,
            arc,
            workplane,
        }
    }
}

impl<L: AsLineSegment> AsConstraintData for EqualLineArcLen<L> {
    fn type_(&self) -> i32 {
        SLVS_C_EQUAL_LINE_ARC_LEN as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle(), self.arc.handle()])
    }
}

impl<L: AsLineSegment> TypeInfo for EqualLineArcLen<L> {
    fn type_of() -> String {
        format!("EqualLineArcLen < {} >", L::type_of())
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for EqualLineArcLen<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            line: Entity::new(value.entityA),
            arc: Entity::new(value.entityB),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
