use std::sync::atomic::{AtomicU32, Ordering};

use crate::bindings;

static NEXT_PARAM_H: AtomicU32 = AtomicU32::new(1);

impl bindings::Slvs_Param {
    fn new(group: bindings::Slvs_hGroup, val: f64) -> Self {
        Self {
            h: NEXT_PARAM_H.fetch_add(1, Ordering::SeqCst),
            group,
            val,
        }
    }
}
