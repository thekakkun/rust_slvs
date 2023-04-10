use crate::binding;

// pub type Param = binding::Slvs_Param;

impl From<binding::Slvs_Param> for binding::Slvs_hParam {
    fn from(value: binding::Slvs_Param) -> Self {
        value.h
    }
}
