use crate::binding;

#[derive(Clone, Copy)]
pub struct Group(pub u32);

impl From<Group> for binding::Slvs_hGroup {
    fn from(value: Group) -> Self {
        value.0
    }
}
