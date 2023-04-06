#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use element::{constraint::Constraint, entity::Entity, group::Group, param::Param, Elements};

mod element;

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

enum Result {
    Okay = binding::SLVS_RESULT_OKAY as isize,
    Inconsistent = binding::SLVS_RESULT_INCONSISTENT as isize,
    DidntConverge = binding::SLVS_RESULT_DIDNT_CONVERGE as isize,
    TooManyUnknowns = binding::SLVS_RESULT_TOO_MANY_UNKNOWNS as isize,
}

pub struct System {
    groups: Elements<Group>,
    params: Elements<Param>,
    entities: Elements<Entity>,
    constraints: Elements<Constraint>,
    dragged: [u32; 4],
    calculateFaileds: bool,
    failed: Vec<binding::Slvs_hConstraint>,
    dof: i32,
    result: Result,
}

impl System {
    pub fn new() -> Self {
        Self {
            groups: Elements::<Group>::default(),
            params: Elements::<Param>::default(),
            entities: Elements::<Entity>::default(),
            constraints: Elements::<Constraint>::default(),
            dragged: [0; 4],
            calculateFaileds: true,
            failed: Vec::<binding::Slvs_hConstraint>::new(),
            dof: 0,
            result: Result::Okay,
        }
    }

    pub fn add_point_3d(&mut self, group: Group, x: f64, y: f64, z: f64) -> Entity {
        let x_param = self.params.add(Param::new(group, x));
        let y_param = self.params.add(Param::new(group, y));
        let z_param = self.params.add(Param::new(group, z));

        self.entities.add(Entity::new_point_3d(
            group.into(),
            x_param.into(),
            y_param.into(),
            z_param.into(),
        ))
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

//     pub fn add_group(&mut self) -> Weak<Group> {
//         self.groups.add(Group::default())
//     }

// }

pub fn solve(sys: System, h: Group) {
    unimplemented!();
    // unsafe { bindings::Slvs_Solve(sys, hg) };
}

pub fn example_3d() -> f64 {
    println!("Running 3D example");

    let g: binding::Slvs_hGroup = 1;

    let x1 = binding::Slvs_Param {
        h: 1,
        group: g,
        val: 10.0,
    };
    let y1 = binding::Slvs_Param {
        h: 2,
        group: g,
        val: 10.0,
    };
    let z1 = binding::Slvs_Param {
        h: 3,
        group: g,
        val: 10.0,
    };
    let p1 = binding::Slvs_Entity {
        h: 101,
        group: g,
        type_: binding::SLVS_E_POINT_IN_3D as i32,
        wrkpl: binding::SLVS_FREE_IN_3D,
        point: [0; 4],
        normal: 0,
        distance: 0,
        param: [1, 2, 3, 0],
    };

    println!("  Created point 1 at: ({}, {}, {})", x1.val, y1.val, z1.val);

    let x2 = binding::Slvs_Param {
        h: 4,
        group: g,
        val: 20.0,
    };
    let y2 = binding::Slvs_Param {
        h: 5,
        group: g,
        val: 20.0,
    };
    let z2 = binding::Slvs_Param {
        h: 6,
        group: g,
        val: 20.0,
    };
    let p2 = binding::Slvs_Entity {
        h: 102,
        group: g,
        type_: binding::SLVS_E_POINT_IN_3D as i32,
        wrkpl: binding::SLVS_FREE_IN_3D,
        point: [0; 4],
        normal: 0,
        distance: 0,
        param: [4, 5, 6, 0],
    };

    println!("  Created point 2 at: ({}, {}, {})", x2.val, y2.val, z2.val);

    let c1 = binding::Slvs_Constraint {
        h: 1,
        group: g,
        type_: binding::SLVS_C_PT_PT_DISTANCE as i32,
        wrkpl: binding::SLVS_FREE_IN_3D,
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

    let mut sys = binding::Slvs_System {
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

    unsafe { binding::Slvs_Solve(&mut sys, g) }

    if sys.result == binding::SLVS_RESULT_OKAY.try_into().unwrap() {
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
