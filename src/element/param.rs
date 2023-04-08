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

// #[derive(Clone, Copy, Debug)]
// pub struct ParamH(binding::Slvs_hParam);

// impl ParamH {
//     pub(crate) fn new() -> Self {
//         Self(NEXT_PARAM_H.fetch_add(1, Ordering::SeqCst))
//     }
// }

// impl From<Param> for ParamH {
//     fn from(value: Param) -> Self {
//         Self(value.h)
//     }
// }
// impl From<Handle> for ParamH {
//     fn from(value: Handle) -> Self {
//         if let Handle::Param(h) = value {
//             h
//         } else {
//             panic!("Only Handle::Param can be converted into ParamH")
//         }
//     }
// }

// impl From<ParamH> for u32 {
//     fn from(value: ParamH) -> Self {
//         value.0
//     }
// }

// impl From<Param> for binding::Slvs_hParam {
//     fn from(value: Param) -> Self {
//         value.h
//     }
// }

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
