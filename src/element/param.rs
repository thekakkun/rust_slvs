use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

use super::group::GroupH;

static NEXT_PARAM_H: AtomicU32 = AtomicU32::new(1);

pub struct ParamH(binding::Slvs_hParam);

impl ParamH {
    fn new() -> Self {
        Self(NEXT_PARAM_H.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for ParamH {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ParamH> for binding::Slvs_hParam {
    fn from(value: ParamH) -> Self {
        value.0
    }
}

impl binding::Slvs_Param {
    fn new(group: GroupH, val: f64) -> Self {
        Self {
            h: ParamH::default().into(),
            group: group.into(),
            val,
        }
    }
}
