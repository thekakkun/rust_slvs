/*!
Each [entity][`crate::entity`] and [constraint][`crate::constraint`] is assigned to a [`Group`].

When solving the system, the solver will only modify elements belonging to the group specified.
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
