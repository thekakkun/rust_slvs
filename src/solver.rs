use crate::{
    bindings::{
        SLVS_RESULT_DIDNT_CONVERGE, SLVS_RESULT_INCONSISTENT, SLVS_RESULT_TOO_MANY_UNKNOWNS,
    },
    constraint::AsConstraint,
};

#[derive(Debug)]
pub struct SolveOkay {
    pub dof: i32,
}

#[derive(Debug)]
pub struct SolveFail {
    pub dof: i32,
    pub reason: FailReason,
    pub failed_constraints: Vec<Box<dyn AsConstraint>>,
}

impl SolveFail {
    pub fn constraint_did_fail<T: AsConstraint>(&self, constraint: &T) -> bool {
        self.failed_constraints
            .iter()
            .map(|constraint| constraint.handle())
            .any(|constraint_h| constraint_h == constraint.handle())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
