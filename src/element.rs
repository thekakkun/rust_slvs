pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

pub(crate) struct Elements<T>(Vec<T>);

impl<T: Copy> Elements<T> {
    fn new() -> Self {
        Elements(Vec::new())
    }

    pub(crate) fn add(&mut self, element: T) -> T {
        self.0.push(element);
        element
    }
}

impl<T: Copy> Default for Elements<T> {
    fn default() -> Self {
        Self::new()
    }
}
