#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{iter::zip, mem::MaybeUninit};

use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::system::System;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

////////////////////////////////////////////////////////////////////////////////
// Param
////////////////////////////////////////////////////////////////////////////////

impl Serialize for Slvs_Param {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Slvs_Param", 3)?;
        state.serialize_field("h", &self.h)?;
        state.serialize_field("group", &self.group)?;
        state.serialize_field("val", &self.val)?;
        state.end()
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

    pub(crate) fn set_group(&mut self, group: Slvs_hGroup) {
        self.group = group;
    }

    pub(crate) fn set_workplane(&mut self, workplane: Slvs_hEntity) {
        self.wrkpl = workplane;
    }

    pub(crate) fn set_point(&mut self, points: Vec<Slvs_hEntity>) {
        for (i, point) in points.iter().enumerate() {
            self.point[i] = *point;
        }
    }

    pub(crate) fn set_normal(&mut self, normal: Slvs_hEntity) {
        self.normal = normal;
    }

    pub(crate) fn set_distance(&mut self, distance: Slvs_hEntity) {
        self.distance = distance;
    }

    pub(crate) fn set_param(&mut self, param: Vec<Slvs_hParam>) {
        for (i, param_h) in param.iter().enumerate() {
            self.param[i] = *param_h;
        }
    }
}

impl Serialize for Slvs_Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Slvs_Entity", 8)?;
        state.serialize_field("h", &self.h)?;
        state.serialize_field("group", &self.group)?;
        state.serialize_field("type_", &self.type_)?;
        state.serialize_field("wrkpl", &self.wrkpl)?;
        state.serialize_field("point", &self.point)?;
        state.serialize_field("normal", &self.normal)?;
        state.serialize_field("distance", &self.distance)?;
        state.serialize_field("param", &self.param)?;
        state.end()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Constraint
////////////////////////////////////////////////////////////////////////////////

impl Slvs_Constraint {
    pub(crate) fn new(h: Slvs_hEntity, group: Slvs_hGroup, type_: i32) -> Self {
        Self {
            h,
            group,
            type_,
            wrkpl: SLVS_FREE_IN_3D,
            valA: 0.0,
            ptA: 0,
            ptB: 0,
            entityA: 0,
            entityB: 0,
            entityC: 0,
            entityD: 0,
            other: 0,
            other2: 0,
        }
    }

    pub(crate) fn set_group(&mut self, group: Slvs_hGroup) {
        self.group = group;
    }

    pub(crate) fn set_workplane(&mut self, workplane: Slvs_hEntity) {
        self.wrkpl = workplane;
    }

    pub(crate) fn set_val(&mut self, val: f64) {
        self.valA = val;
    }

    pub(crate) fn set_points(&mut self, points: Vec<Slvs_hEntity>) {
        zip([&mut self.ptA, &mut self.ptB], points).for_each(|(data, point)| {
            *data = point;
        });
    }

    pub(crate) fn set_entities(&mut self, entities: Vec<Slvs_hEntity>) {
        zip(
            [
                &mut self.entityA,
                &mut self.entityB,
                &mut self.entityC,
                &mut self.entityD,
            ],
            entities,
        )
        .for_each(|(data, entities)| {
            *data = entities;
        });
    }

    pub(crate) fn set_others(&mut self, others: [bool; 2]) {
        self.other = others[0] as _;
        self.other2 = others[1] as _;
    }
}

impl Serialize for Slvs_Constraint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Slvs_Constraint", 13)?;
        state.serialize_field("h", &self.h)?;
        state.serialize_field("group", &self.group)?;
        state.serialize_field("type_", &self.type_)?;
        state.serialize_field("wrkpl", &self.wrkpl)?;
        state.serialize_field("valA", &self.valA)?;
        state.serialize_field("ptA", &self.ptA)?;
        state.serialize_field("ptB", &self.ptB)?;
        state.serialize_field("entityA", &self.entityA)?;
        state.serialize_field("entityB", &self.entityB)?;
        state.serialize_field("entityC", &self.entityC)?;
        state.serialize_field("entityD", &self.entityD)?;
        state.serialize_field("other", &self.other)?;
        state.serialize_field("other2", &self.h)?;
        state.end()
    }
}

////////////////////////////////////////////////////////////////////////////////
// System
////////////////////////////////////////////////////////////////////////////////

impl Slvs_System {
    pub(super) fn from(system: &mut System, failed_handles: &mut Vec<Slvs_hConstraint>) -> Self {
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

////////////////////////////////////////////////////////////////////////////////
// Quaternion
////////////////////////////////////////////////////////////////////////////////

/// Get the basis vector `U` from a quaternion.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn quaternion_u(quaternion: [f64; 4]) -> [f64; 3] {
    let [qw, qx, qy, qz] = quaternion;

    let mut x = MaybeUninit::<f64>::uninit();
    let mut y = MaybeUninit::<f64>::uninit();
    let mut z = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_QuaternionU(
            qw,
            qx,
            qy,
            qz,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            z.as_mut_ptr(),
        );

        [x.assume_init(), y.assume_init(), z.assume_init()]
    }
}

/// Get the basis vector `V` from a quaternion.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn quaternion_v(quaternion: [f64; 4]) -> [f64; 3] {
    let [qw, qx, qy, qz] = quaternion;

    let mut x = MaybeUninit::<f64>::uninit();
    let mut y = MaybeUninit::<f64>::uninit();
    let mut z = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_QuaternionV(
            qw,
            qx,
            qy,
            qz,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            z.as_mut_ptr(),
        );

        [x.assume_init(), y.assume_init(), z.assume_init()]
    }
}

/// Get the normal vector `N` from a quaternion.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn quaternion_n(quaternion: [f64; 4]) -> [f64; 3] {
    let [qw, qx, qy, qz] = quaternion;

    let mut x = MaybeUninit::<f64>::uninit();
    let mut y = MaybeUninit::<f64>::uninit();
    let mut z = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_QuaternionN(
            qw,
            qx,
            qy,
            qz,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            z.as_mut_ptr(),
        );

        [x.assume_init(), y.assume_init(), z.assume_init()]
    }
}

/// Compute a unit quaternion from two basis vectors.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn make_quaternion(basis_vec_1: [f64; 3], basic_vec_2: [f64; 3]) -> [f64; 4] {
    let [ux, uy, uz] = basis_vec_1;
    let [vx, vy, vz] = basic_vec_2;

    let mut qw = MaybeUninit::<f64>::uninit();
    let mut qx = MaybeUninit::<f64>::uninit();
    let mut qy = MaybeUninit::<f64>::uninit();
    let mut qz = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_MakeQuaternion(
            ux,
            uy,
            uz,
            vx,
            vy,
            vz,
            qw.as_mut_ptr(),
            qx.as_mut_ptr(),
            qy.as_mut_ptr(),
            qz.as_mut_ptr(),
        );
        [
            qw.assume_init(),
            qx.assume_init(),
            qy.assume_init(),
            qz.assume_init(),
        ]
    }
}
