use serde::{Deserialize, Serialize};

use crate::{
    bindings::{
        SLVS_RESULT_DIDNT_CONVERGE, SLVS_RESULT_INCONSISTENT, SLVS_RESULT_TOO_MANY_UNKNOWNS,
    },
    constraint::AsConstraintHandle,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SolveOkay {
    pub dof: i32,
}

#[derive(Debug)]
pub struct SolveFail {
    pub dof: i32,
    pub reason: FailReason,
    pub failed_constraints: Vec<Box<dyn AsConstraintHandle>>,
}

impl SolveFail {
    pub fn constraint_failed(&self, constraint: impl AsConstraintHandle) -> bool {
        self.failed_constraints
            .iter()
            .any(|constraint_h| constraint_h.handle() == constraint.handle())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FailReason {
    Inconsistent,
    DidntConverge,
    TooManyUnknowns,
}

impl TryFrom<i32> for FailReason {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, &'static str> {
        match value as _ {
            SLVS_RESULT_INCONSISTENT => Ok(Self::Inconsistent),
            SLVS_RESULT_DIDNT_CONVERGE => Ok(Self::DidntConverge),
            SLVS_RESULT_TOO_MANY_UNKNOWNS => Ok(Self::TooManyUnknowns),
            _ => Err("Result must be of values 1, 2, or 3."),
        }
    }
}
