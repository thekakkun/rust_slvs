use std::sync::atomic::AtomicU32;

pub(crate) static NEXT_GROUP_H: AtomicU32 = AtomicU32::new(1);

// pub(crate) struct Group(binding::Slvs_hGroup);

// impl Group {
//     fn new() -> Self {
//         Group(NEXT_GROUP_H.fetch_add(1, Ordering::SeqCst))
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub struct GroupH(binding::Slvs_hGroup);

// impl GroupH {
//     pub(crate) fn new() -> Self {
//         Self(NEXT_GROUP_H.fetch_add(1, Ordering::SeqCst))
//     }
// }

// impl From<Handle> for binding::Slvs_hGroup {
//     fn from(value: Handle) -> Self {
//         if let Handle::Group(h) = value {
//             h
//         } else {
//             panic!("Only Handle::Group can be converted into GroupH")
//         }
//     }
// }

// impl From<GroupH> for Handle {
//     fn from(value: GroupH) -> Self {
//         Self::Group(value)
//     }
// }

// impl From<GroupH> for u32 {
//     fn from(value: GroupH) -> Self {
//         value.0
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Group {
//     pub h: binding::Slvs_hGroup,
// }

// impl Group {
//     pub fn new() -> Self {
//         Self {
//             h: NEXT_GROUP_H.fetch_add(1, Ordering::SeqCst),
//         }
//     }
// }

// impl Default for Group {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl From<Group> for binding::Slvs_hGroup {
//     fn from(value: Group) -> Self {
//         value.h
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::binding;

//     use super::Group;

//     #[test]
//     fn incremental_handle_generated() {
//         // handle starts from 1
//         let mut g = Group::new();
//         assert_eq!(binding::Slvs_hGroup::from(g), 1);

//         // increments to 2
//         g = Group::new();
//         assert_eq!(binding::Slvs_hGroup::from(g), 2);

//         // increments to 3
//         g = Group::new();
//         assert_eq!(binding::Slvs_hGroup::from(g), 3);
//     }
// }
