use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_CUBIC_LINE_TANGENT},
    element::AsHandle,
    entity::{AsCubic, AsLineSegment, EntityHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub cubic: EntityHandle<C>,
    pub line: EntityHandle<L>,
    pub to_beginning: bool,
}

impl<C, L> CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        arc: EntityHandle<C>,
        line: EntityHandle<L>,
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

impl<C, L> From<Slvs_Constraint> for CubicLineTangent<C, L>
where
    C: AsCubic,
    L: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            workplane: EntityHandle::new(value.wrkpl),
            cubic: EntityHandle::new(value.entityA),
            line: EntityHandle::new(value.entityB),
            to_beginning: value.other != 0,
        }
    }
}
