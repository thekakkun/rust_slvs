use crate::binding;

pub type Param = binding::Slvs_Param;

impl From<Param> for binding::Slvs_hParam {
    fn from(value: Param) -> Self {
        value.h
    }
}
