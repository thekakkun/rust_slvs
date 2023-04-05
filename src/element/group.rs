use crate::bindings;

pub struct Group(bindings::Slvs_hGroup);

impl From<bindings::Slvs_hGroup> for Group {
    fn from(value: bindings::Slvs_hGroup) -> Self {
        Group(value)
    }
}
