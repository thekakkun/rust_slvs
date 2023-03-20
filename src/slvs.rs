#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use self::{constraint::Constraint, entity::Entity, parameter::Param};

mod constraint;
mod entity;
mod parameter;

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct System {
    param: Vec<Param>,
    entity: Vec<Entity>,
    constraint: Vec<Constraint>,
    dragged: [u32; 4],
    calculateFaileds: bool,
    failed: Vec<bindings::Slvs_hConstraint>,
    dof: i32,
    result: i32,
}

pub fn solve() {
    unimplemented!();
    // unsafe { bindings::Slvs_Solve(sys, hg) };
}
