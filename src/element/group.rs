use super::Handle;

#[derive(Default)]
pub struct Group(pub u32);

impl Handle for Group {
    fn get_handle(&self) -> u32 {
        self.0
    }

    fn set_handle(&mut self, h: u32) {
        self.0 = h;
    }
}
