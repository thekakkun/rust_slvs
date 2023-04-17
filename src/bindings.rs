#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{
    constraint::{AsConstraint, Constraint},
    entity::{AsEntity, Entity},
    Group, System,
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

    pub(crate) fn point(&mut self, points: Vec<Slvs_hEntity>) {
        for (i, point) in points.iter().enumerate() {
            self.point[i] = *point;
        }
    }

    pub(crate) fn normal(&mut self, normal: Slvs_hEntity) {
        self.normal = normal;
    }

    pub(crate) fn distance(&mut self, distance: Slvs_hEntity) {
        self.distance = distance;
    }

    pub(crate) fn param(&mut self, param: Vec<Slvs_hParam>) {
        for (i, param_h) in param.iter().enumerate() {
            self.param[i] = *param_h;
        }
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

////////////////////////////////////////////////////////////////////////////////
// System
////////////////////////////////////////////////////////////////////////////////

impl Slvs_System {
    pub(super) fn from(system: &mut System) -> Self {
        let mut failed_handles: Vec<Slvs_hConstraint> = vec![0; system.constraints.list.len()];

        Slvs_System {
            param: system.params.list.as_mut_ptr(),
            params: system.params.list.len() as _,
            entity: system.entities.list.as_mut_ptr(),
            entities: system.entities.list.len() as _,
            constraint: system.constraints.list.as_mut_ptr(),
            constraints: system.constraints.list.len() as _,
            dragged: system.dragged,
            calculateFaileds: system.calculate_faileds as _,
            failed: failed_handles.as_mut_ptr(),
            faileds: failed_handles.len() as _,
            dof: 0,
            result: 0,
        }
    }
}
