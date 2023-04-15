use crate::{
    bindings,
    entity::{Entity, PointIn3d},
};

use super::{AsConstraint, Constraint, SomeConstraint};

pub enum PtPtDistance {
    _2d {
        val: f64,
        workplane: Entity<PointIn3d>, // Not really the correct type. fix later.
        point_a: Entity<PointIn3d>,
        point_b: Entity<PointIn3d>,
    },
    _3d {
        val: f64,
        point_a: Entity<PointIn3d>,
        point_b: Entity<PointIn3d>,
    },
}

impl AsConstraint for PtPtDistance {
    fn type_(&self) -> bindings::Slvs_hConstraint {
        bindings::SLVS_C_PT_PT_DISTANCE
    }

    fn workplane(&self) -> Option<bindings::Slvs_hEntity> {
        None // TODO: necessary for 2d distances.
    }

    fn val(&self) -> f64 {
        match self {
            PtPtDistance::_2d { val, .. } | PtPtDistance::_3d { val, .. } => *val,
        }
    }

    fn point(&self) -> [Option<bindings::Slvs_hEntity>; 2] {
        match self {
            PtPtDistance::_2d {
                point_a, point_b, ..
            }
            | PtPtDistance::_3d {
                point_a, point_b, ..
            } => [Some((*point_a).into()), Some((*point_b).into())],
        }
    }

    fn entity(&self) -> [Option<bindings::Slvs_hEntity>; 4] {
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
