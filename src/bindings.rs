#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{
    constraint::{AsConstraint, Constraint},
    entity::{AsEntity, Entity},
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

impl Slvs_Entity {
    pub(crate) fn new(h: Slvs_hEntity, group: Slvs_hGroup, type_: i32) -> Self {
        Self {
            h,
            group,
            type_,
            wrkpl: SLVS_FREE_IN_3D,
            point: [0; 4],
            normal: 0,
            distance: 0,
            param: [0; 4],
        }
    }

    pub(crate) fn workplane(&mut self, workplane: Slvs_hEntity) {
        self.wrkpl = workplane;
    }

    pub(crate) fn point(&mut self, point: [Slvs_hEntity; 4]) {
        self.point = point;
    }

    pub(crate) fn normal(&mut self, normal: Slvs_hEntity) {
        self.normal = normal;
    }

    pub(crate) fn distance(&mut self, distance: Slvs_hEntity) {
        self.distance = distance;
    }

    pub(crate) fn param(&mut self, param: [Slvs_hEntity; 4]) {
        self.param = param;
    }
}

impl<T: AsEntity> From<Entity<T>> for Slvs_hEntity {
    fn from(value: Entity<T>) -> Self {
        value.handle
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
