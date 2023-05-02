use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_CUBIC_LINE_TANGENT},
    element::{AsHandle, TypeInfo},
    entity::{AsCubic, AsLineSegment, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    pub group: Group,
    pub workplane: Entity<Workplane>,
    pub cubic: Entity<C>,
    pub line: Entity<L>,
    pub to_beginning: bool,
}

impl<C, L> CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    pub fn new(
        group: Group,
        workplane: Entity<Workplane>,
        arc: Entity<C>,
        line: Entity<L>,
        to_beginning: bool,
    ) -> Self {
        Self {
            group,
            workplane,
            cubic: arc,
            line,
            to_beginning,
        }
    }
}

impl<C, L> AsConstraintData for CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_CUBIC_LINE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.cubic.handle(), self.line.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_beginning, false]
    }
}

impl<C, L> TypeInfo for CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    fn type_of() -> String {
        format!("CubicLineTangent < {} , {} >", C::type_of(), L::type_of())
    }
}

impl<C, L> From<Slvs_Constraint> for CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            workplane: Entity::new(value.wrkpl),
            cubic: Entity::new(value.entityA),
            line: Entity::new(value.entityB),
            to_beginning: value.other != 0,
        }
    }
}
