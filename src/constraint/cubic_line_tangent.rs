use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_CUBIC_LINE_TANGENT},
    element::{AsHandle, TypeInfo},
    entity::{AsCubic, AsLineSegment, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    workplane: Entity<Workplane>,
    cubic: Entity<C>,
    line: Entity<L>,
    to_beginning: bool,
}

impl<C, L> CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    pub fn new(
        workplane: Entity<Workplane>,
        arc: Entity<C>,
        line: Entity<L>,
        to_beginning: bool,
    ) -> Self {
        Self {
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
            workplane: Entity::new(value.wrkpl),
            cubic: Entity::new(value.entityA),
            line: Entity::new(value.entityB),
            to_beginning: value.other != 0,
        }
    }
}