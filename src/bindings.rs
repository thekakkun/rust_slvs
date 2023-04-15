#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{
    constraint::{AsConstraint, Constraint},
    entity::{AsEntity, Entity, SomeEntity},
    Group,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

////////////////////////////////////////////////////////////////////////////////
// Group
////////////////////////////////////////////////////////////////////////////////

impl From<Group> for Slvs_hGroup {
    fn from(value: Group) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entity
////////////////////////////////////////////////////////////////////////////////

impl<T: AsEntity> From<Entity<T>> for Slvs_hEntity {
    fn from(value: Entity<T>) -> Self {
        value.handle
    }
}

impl From<SomeEntity> for Slvs_hEntity {
    fn from(value: SomeEntity) -> Self {
        match value {
            SomeEntity::PointIn3d(e) => e.handle,
            SomeEntity::LineSegment(e) => e.handle,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Constraint
////////////////////////////////////////////////////////////////////////////////

impl<T: AsConstraint> From<Constraint<T>> for Slvs_hConstraint {
    fn from(value: Constraint<T>) -> Self {
        value.handle
    }
}
