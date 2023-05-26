#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::mem::MaybeUninit;

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
