pub struct Group(u32);

impl Group {
    pub fn new(handle: u32) -> Self {
        Self(handle)
    }
}
