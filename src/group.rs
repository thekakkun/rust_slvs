use serde::{Deserialize, Serialize};

use crate::element::AsHandle;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Group(pub u32);

impl AsHandle for Group {
    fn handle(&self) -> u32 {
        self.0
    }
}
