use crate::binding;

use self::{constraint::Constraint, entity::Entity, param::Param};

pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

#[derive(Clone, Copy, Debug)]
pub enum Handle {
    Group(binding::Slvs_hGroup),
    Param(binding::Slvs_hParam),
    Entity(binding::Slvs_hEntity),
    Constraint(binding::Slvs_hConstraint),
}

impl From<Param> for Handle {
    fn from(value: Param) -> Self {
        Self::Param(value.h)
    }
}

impl From<Entity> for Handle {
    fn from(value: Entity) -> Self {
        Self::Entity(value.h)
    }
}

impl From<Constraint> for Handle {
    fn from(value: Constraint) -> Self {
        Self::Constraint(value.h)
    }
}

impl From<Handle> for u32 {
    fn from(value: Handle) -> Self {
        match value {
            Handle::Group(h) | Handle::Param(h) | Handle::Entity(h) | Handle::Constraint(h) => h,
        }
    }
}

// pub(crate) type Elements<T> = Vec<T>;

pub(crate) trait PushReturn<T: Copy + Into<Handle>> {
    fn push_return(&mut self, element: T) -> Handle;
}

impl<T> PushReturn<T> for Vec<T>
where
    T: Copy + Into<Handle>,
{
    fn push_return(&mut self, element: T) -> Handle {
        self.push(element);
        element.into()
    }
}

// pub(crate) struct Elements<T>(pub(crate) Vec<T>);

// impl<T> Elements<T> {
//     fn new() -> Self {
//         Elements(Vec::new())
//     }

//     pub(crate) fn add(&mut self, element: T) -> &T {
//         self.0.push(element);
//         self.0.last().unwrap()
//     }
// }

// impl<T> Default for Elements<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
