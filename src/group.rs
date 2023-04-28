use crate::element::AsHandle;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group(pub(super) u32);

impl AsHandle for Group {
    fn handle(&self) -> u32 {
        self.0
    }
}
