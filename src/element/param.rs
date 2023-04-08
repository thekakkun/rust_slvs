use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

static NEXT_PARAM_H: AtomicU32 = AtomicU32::new(1);

pub type Param = binding::Slvs_Param;

impl Param {
    pub(crate) fn new(group: binding::Slvs_hGroup, val: f64) -> Self {
        Self {
            h: NEXT_PARAM_H.fetch_add(1, Ordering::SeqCst),
            group,
            val,
        }
    }
}
