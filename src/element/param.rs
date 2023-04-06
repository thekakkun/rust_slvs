use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

use super::group::Group;

static NEXT_PARAM_H: AtomicU32 = AtomicU32::new(1);

pub type Param = binding::Slvs_Param;

impl Param {
    pub(crate) fn new(group: Group, val: f64) -> Self {
        Self {
            h: NEXT_PARAM_H.fetch_add(1, Ordering::SeqCst),
            group: group.into(),
            val,
        }
    }
}

impl From<Param> for binding::Slvs_hParam {
    fn from(value: Param) -> Self {
        value.h
    }
}

// #[derive(Clone, Copy)]
// pub struct ParamH(pub binding::Slvs_hParam);

// impl ParamH {
//     fn new() -> Self {
//         Self(NEXT_PARAM_H.fetch_add(1, Ordering::SeqCst))
//     }
// }

// impl Default for ParamH {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl From<ParamH> for binding::Slvs_hParam {
//     fn from(value: ParamH) -> Self {
//         value.0
//     }
// }

// impl Elements<Param> {
//     pub fn add(&mut self, param: Param) -> ParamH {
//         self.0.push(param);
//         ParamH(param.h)
//     }
// }
