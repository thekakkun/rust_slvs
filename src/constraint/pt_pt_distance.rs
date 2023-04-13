use crate::{
    binding,
    entity::{Entity, PointIn3d, SomeEntity},
};

use super::{AsConstraint, Constraint, SomeConstraint};

pub enum PtPtDistance {
    _2d {
        val: f64,
        workplane: Entity<PointIn3d>, // Not really the correct type. fix later.
        pt_a: Entity<PointIn3d>,
        pt_b: Entity<PointIn3d>,
    },
    _3d {
        val: f64,
        pt_a: Entity<PointIn3d>,
        pt_b: Entity<PointIn3d>,
    },
}

impl AsConstraint for PtPtDistance {
    fn type_(&self) -> binding::Slvs_hConstraint {
        binding::SLVS_C_PT_PT_DISTANCE
    }

    fn workplane(&self) -> Option<binding::Slvs_hEntity> {
        None // TODO: necessary for 2d distances.
             // self.workplane.map(|e| e.into())
    }

    fn val(&self) -> f64 {
        match self {
            PtPtDistance::_2d { val, .. } | PtPtDistance::_3d { val, .. } => *val,
        }
    }

    fn point(&self) -> [Option<binding::Slvs_hEntity>; 2] {
        match self {
            PtPtDistance::_2d { pt_a, pt_b, .. } | PtPtDistance::_3d { pt_a, pt_b, .. } => {
                [Some((*pt_a).into()), Some((*pt_b).into())]
            }
        }
    }

    fn entity(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [None; 4]
    }

    fn other(&self) -> [bool; 2] {
        [false; 2]
    }
}

impl TryFrom<SomeConstraint> for Constraint<PtPtDistance> {
    type Error = &'static str;

    fn try_from(value: SomeConstraint) -> Result<Self, Self::Error> {
        if let SomeConstraint::PtPtDistance(constraint) = value {
            Ok(constraint)
        } else {
            Err("Expected SomeConstraint::PtPtDistance")
        }
    }
}

impl From<Constraint<PtPtDistance>> for SomeConstraint {
    fn from(value: Constraint<PtPtDistance>) -> Self {
        SomeConstraint::PtPtDistance(value)
    }
}
