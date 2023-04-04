#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::rc::Weak;

use element::{constraint::Constraint, entity::Entity, group::Group, param::Param, Elements};

mod element;

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct System {
    groups: Elements<Group>,
    params: Elements<Param>,
    entities: Elements<Entity>,
    constraints: Elements<Constraint>,
    dragged: [u32; 4],
    calculateFaileds: bool,
    failed: Vec<Weak<Constraint>>,
    dof: i32,
    result: i32,
}

impl System {
    pub fn add_group(&mut self) -> Weak<Group> {
        self.groups.add(Group::default())
    }

    pub fn add_point_3d(&mut self, group: &Weak<Group>, x: f64, y: f64, z: f64) -> Weak<Entity> {
        let x_param = self.params.add(Param::new(group, x));
        let y_param = self.params.add(Param::new(group, y));
        let z_param = self.params.add(Param::new(group, z));

        self.entities
            .add(Entity::new_point_3d(group, &x_param, &y_param, &z_param))
    }
}

pub fn solve() {
    unimplemented!();
    // unsafe { bindings::Slvs_Solve(sys, hg) };
}

pub fn example_3d() -> f64 {
    println!("Running 3D example");

    let g: bindings::Slvs_hGroup = 1;

    let x1 = bindings::Slvs_Param {
        h: 1,
        group: g,
        val: 10.0,
    };
    let y1 = bindings::Slvs_Param {
        h: 2,
        group: g,
        val: 10.0,
    };
    let z1 = bindings::Slvs_Param {
        h: 3,
        group: g,
        val: 10.0,
    };
    let p1 = bindings::Slvs_Entity {
        h: 101,
        group: g,
        type_: bindings::SLVS_E_POINT_IN_3D as i32,
        wrkpl: bindings::SLVS_FREE_IN_3D,
        point: [0; 4],
        normal: 0,
        distance: 0,
        param: [1, 2, 3, 0],
    };

    println!("  Created point 1 at: ({}, {}, {})", x1.val, y1.val, z1.val);

    let x2 = bindings::Slvs_Param {
        h: 4,
        group: g,
        val: 20.0,
    };
    let y2 = bindings::Slvs_Param {
        h: 5,
        group: g,
        val: 20.0,
    };
    let z2 = bindings::Slvs_Param {
        h: 6,
        group: g,
        val: 20.0,
    };
    let p2 = bindings::Slvs_Entity {
        h: 102,
        group: g,
        type_: bindings::SLVS_E_POINT_IN_3D as i32,
        wrkpl: bindings::SLVS_FREE_IN_3D,
        point: [0; 4],
        normal: 0,
        distance: 0,
        param: [4, 5, 6, 0],
    };

    println!("  Created point 2 at: ({}, {}, {})", x2.val, y2.val, z2.val);

    let c1 = bindings::Slvs_Constraint {
        h: 1,
        group: g,
        type_: bindings::SLVS_C_PT_PT_DISTANCE as i32,
        wrkpl: bindings::SLVS_FREE_IN_3D,
        valA: 30.0,
        ptA: 101,
        ptB: 102,
        entityA: 0,
        entityB: 0,
        entityC: 0,
        entityD: 0,
        other: 0,
        other2: 0,
    };

    println!("  Constraint created: Distance between points should be 30.0 units");

    let mut param_list = vec![x1, y1, z1, x2, y2, z2];
    let mut entity_list = vec![p1, p2];
    let mut constraint_list = vec![c1];
    let mut failed_list = vec![0; 50];

    let mut sys = bindings::Slvs_System {
        param: param_list.as_mut_ptr(),
        params: param_list.len() as i32,
        entity: entity_list.as_mut_ptr(),
        entities: entity_list.len() as i32,
        constraint: constraint_list.as_mut_ptr(),
        constraints: constraint_list.len() as i32,
        dragged: [4, 5, 6, 0],
        calculateFaileds: 1,
        failed: failed_list.as_mut_ptr(),
        faileds: failed_list.len() as i32,
        dof: 0,
        result: 0,
    };

    unsafe { bindings::Slvs_Solve(&mut sys, g) }

    if sys.result == bindings::SLVS_RESULT_OKAY.try_into().unwrap() {
        println!("  Constraints solved");
        println!(
            "    Point 1 now at : ({:.3}, {:.3}, {:.3})",
            param_list[0].val, param_list[1].val, param_list[2].val
        );
        println!(
            "    Point 2 now at : ({:.3}, {:.3}, {:.3})",
            param_list[3].val, param_list[4].val, param_list[5].val
        );
    } else {
        println!("  Solve failed");
    };

    param_list[0].val
}
