/*!
Each [entity][`crate::entity`] and [constraint][`crate::constraint`] is assigned to a [`Group`].

A group is a set of entities and constraints that is solved simultaneously. In a
parametric CAD system, a single group would typically correspond to a single sketch.
Constraints within a group may refer to entities outside that group, but only the
entities within that group will be modified by the solver.

In this way, a sketch can reference elements drawn previously, while ensuring that
modifications to the system do not propogate backwards into previously defined elements.
*/

use serde::{Deserialize, Serialize};

use crate::element::AsHandle;

/// A wrapper for the group handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group(pub u32);

impl AsHandle for Group {
    fn handle(&self) -> u32 {
        self.0
    }
}
