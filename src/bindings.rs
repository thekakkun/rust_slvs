#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
