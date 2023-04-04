pub(crate) struct Group(pub u32);

impl Group {
    pub(crate) fn new(h: u32) -> Self {
        Self(h)
    }
}

// impl Handle for Group {
//     fn get_handle(&self) -> u32 {
//         self.0
//     }

//     fn set_handle(&mut self, h: u32) {
//         self.0 = h;
//     }
// }
