use crate::binding;

#[derive(Clone, Copy)]
pub struct Param(pub(super) u32);

impl From<Param> for binding::Slvs_hParam {
    fn from(value: Param) -> Self {
        value.0
    }
}