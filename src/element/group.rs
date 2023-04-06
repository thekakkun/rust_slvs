use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

static NEXT_GROUP_H: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Copy)]
pub struct Group {
    h: binding::Slvs_hGroup,
}

impl Group {
    fn new() -> Self {
        Self {
            h: NEXT_GROUP_H.fetch_add(1, Ordering::SeqCst),
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Group> for binding::Slvs_hGroup {
    fn from(value: Group) -> Self {
        value.h
    }
}

#[cfg(test)]
mod tests {
    use crate::binding;

    use super::Group;

    #[test]
    fn incremental_handle_generated() {
        // handle starts from 1
        let mut g = Group::new();
        assert_eq!(binding::Slvs_hGroup::from(g), 1);

        // increments to 2
        g = Group::new();
        assert_eq!(binding::Slvs_hGroup::from(g), 2);

        // increments to 3
        g = Group::new();
        assert_eq!(binding::Slvs_hGroup::from(g), 3);
    }
}
