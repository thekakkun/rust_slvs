use crate::{
    bindings::{
        Slvs_Solve, Slvs_System, Slvs_hConstraint, SLVS_RESULT_DIDNT_CONVERGE,
        SLVS_RESULT_INCONSISTENT, SLVS_RESULT_TOO_MANY_UNKNOWNS,
    },
    constraint::{AsConstraintData, Constraint},
    element::AsElementIdentifier,
    entity::{AsEntityData, Entity},
    Group, System,
};

impl System {
    pub fn set_dragged(&mut self, entity: &Entity<impl AsEntityData>) {
        if let Ok(slvs_entity) = self.slvs_entity(entity.handle()) {
            self.dragged = slvs_entity.param;
        }
    }

    pub fn clear_dragged(&mut self) {
        self.dragged = [0; 4];
    }

    pub fn solve(&mut self, group: &Group) -> Result<SolveOkay, SolveFail> {
        let mut failed_handles: Vec<Slvs_hConstraint> = vec![0; self.constraints.list.len()];
        let mut slvs_system = Slvs_System::from(self, &mut failed_handles);

        unsafe {
            Slvs_Solve(&mut slvs_system, group.handle());
        };

        match FailReason::try_from(slvs_system.result) {
            Ok(fail_reason) => Err(SolveFail {
                dof: slvs_system.dof,
                reason: fail_reason,
                failed_constraints: failed_handles.into_iter().filter(|&c_h| c_h != 0).collect(),
            }),
            Err(_) => Ok(SolveOkay {
                dof: slvs_system.dof,
            }),
        }
    }
}

#[derive(Debug)]
pub struct SolveOkay {
    pub dof: i32,
}

#[derive(Debug)]
pub struct SolveFail {
    pub dof: i32,
    pub reason: FailReason,
    pub failed_constraints: Vec<Slvs_hConstraint>,
}

impl SolveFail {
    pub fn constraint_did_fail<C: AsConstraintData>(&self, constraint: &Constraint<C>) -> bool {
        self.failed_constraints.contains(&constraint.handle())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
