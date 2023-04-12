use crate::{binding, entity::Entity};

use super::{AsConstraint, ConstraintType};

pub struct PtPtDistance {
    pub val: f64,
    pub wrkpl: Option<Entity>,
    pub pt_a: Entity,
    pub pt_b: Entity,
}

impl AsConstraint for PtPtDistance {
    fn type_(&self) -> ConstraintType {
        ConstraintType::PtPtDistance
    }

    fn workplane(&self) -> Option<binding::Slvs_hEntity> {
        self.wrkpl.map(|e| e.into())
    }

    fn val(&self) -> f64 {
        self.val
    }

    fn point(&self) -> [Option<binding::Slvs_hEntity>; 2] {
        [Some(self.pt_a.into()), Some(self.pt_b.into())]
    }

    fn entity(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [None; 4]
    }

    fn other(&self) -> [bool; 2] {
        [false; 2]
    }
}
