// use crate::binding;

// use self::{constraint::Constraint, entity::Entity, param::Param};



// #[derive(Clone, Copy, Debug)]
// pub enum Handle {
//     Group(binding::Slvs_hGroup),
//     Param(binding::Slvs_hParam),
//     Entity(binding::Slvs_hEntity),
//     Constraint(binding::Slvs_hConstraint),
// }

// impl From<Param> for Handle {
//     fn from(value: Param) -> Self {
//         Self::Param(value.h)
//     }
// }

// impl From<Entity> for Handle {
//     fn from(value: Entity) -> Self {
//         Self::Entity(value.h)
//     }
// }

// impl From<Constraint> for Handle {
//     fn from(value: Constraint) -> Self {
//         Self::Constraint(value.h)
//     }
// }

// impl From<Handle> for u32 {
//     fn from(value: Handle) -> Self {
//         match value {
//             Handle::Group(h) | Handle::Param(h) | Handle::Entity(h) | Handle::Constraint(h) => h,
//         }
//     }
// }

// pub(crate) trait PushReturn<T: Copy + Into<Handle>> {
//     fn push_return(&mut self, element: T) -> Handle;
// }

// impl<T> PushReturn<T> for Vec<T>
// where
//     T: Copy + Into<Handle>,
// {
//     fn push_return(&mut self, element: T) -> Handle {
//         self.push(element);
//         element.into()
//     }
// }
