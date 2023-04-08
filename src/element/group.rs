use std::sync::atomic::AtomicU32;

pub(crate) static NEXT_GROUP_H: AtomicU32 = AtomicU32::new(1);
