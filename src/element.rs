pub(super) struct Elements<T> {
    pub(super) list: Vec<T>,
    pub(super) next_h: u32,
}

impl<T> Elements<T> {
    pub(super) fn new() -> Self {
        Self {
            list: Vec::new(),
            next_h: 1,
        }
    }

    pub(super) fn get_next_h(&mut self) -> u32 {
        let current_h = self.next_h;
        self.next_h += 1;

        current_h
    }
}

impl<T> Default for Elements<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) trait AsHandle {
    fn as_handle(&self) -> u32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group(pub(super) u32);

impl AsHandle for Group {
    fn as_handle(&self) -> u32 {
        self.0
    }
}
