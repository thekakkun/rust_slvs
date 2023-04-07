#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::cell::{Ref, RefCell};

use element::{
    constraint::{Constraint, ConstraintType},
    entity::{Entity, EntityType},
    group::Group,
    param::Param,
    Elements, PushLast,
};

pub mod element;

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Clone, Copy)]
enum SolveResult {
    Okay = binding::SLVS_RESULT_OKAY as isize,
    Inconsistent = binding::SLVS_RESULT_INCONSISTENT as isize,
    DidntConverge = binding::SLVS_RESULT_DIDNT_CONVERGE as isize,
    TooManyUnknowns = binding::SLVS_RESULT_TOO_MANY_UNKNOWNS as isize,
}

impl From<i32> for SolveResult {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Okay,
            1 => Self::Inconsistent,
            2 => Self::DidntConverge,
            3 => Self::TooManyUnknowns,
            _ => Self::TooManyUnknowns, // Is this okay? Shouldn't happen, but...
        }
    }
}

pub struct System {
    // groups: Elements<Group>,
    params: Elements<Param>,
    entities: Elements<Entity>,
    constraints: Elements<Constraint>,
    dragged: [u32; 4],
    calculateFaileds: bool,
    failed: Vec<binding::Slvs_hConstraint>,
    dof: i32,
    result: SolveResult,
}

impl System {
    pub fn new() -> Self {
        Self {
            // groups: Elements::<Group>::default(),
            params: RefCell::new(Vec::new()),
            entities: Elements::<Entity>::default(),
            constraints: Elements::<Constraint>::default(),
            dragged: [0; 4],
            calculateFaileds: true,
            failed: Vec::<binding::Slvs_hConstraint>::new(),
            dof: 0,
            result: SolveResult::Okay,
        }
    }

    pub fn solve(&mut self, group: Group) {
        let mut slvs_system = binding::Slvs_System {
            param: self.params.borrow_mut().as_mut_ptr(),
            params: self.params.borrow().len() as i32,
            entity: self.entities.borrow_mut().as_mut_ptr(),
            entities: self.entities.borrow().len() as i32,
            constraint: self.constraints.borrow_mut().as_mut_ptr(),
            constraints: self.entities.borrow().len() as i32,
            dragged: self.dragged,
            calculateFaileds: self.calculateFaileds as i32,
            failed: Vec::with_capacity(self.entities.borrow().len()).as_mut_ptr(),
            faileds: self.entities.borrow().len() as i32,
            dof: self.dof,
            result: self.result as i32,
        };
        unsafe {
            binding::Slvs_Solve(&mut slvs_system, group.into());

            // self.params.replace(Vec::from_raw_parts(
            //     slvs_system.param,
            //     slvs_system.params.try_into().unwrap(),
            //     slvs_system.params.try_into().unwrap(),
            // ));
            // self.entities.replace(Vec::from_raw_parts(
            //     slvs_system.entity,
            //     slvs_system.entities.try_into().unwrap(),
            //     slvs_system.entities.try_into().unwrap(),
            // ));
            // self.constraints.replace(Vec::from_raw_parts(
            //     slvs_system.constraint,
            //     slvs_system.constraints.try_into().unwrap(),
            //     slvs_system.constraints.try_into().unwrap(),
            // ));
            // self.failed = Vec::from_raw_parts(
            //     slvs_system.failed,
            //     slvs_system.faileds.try_into().unwrap(),
            //     slvs_system.faileds.try_into().unwrap(),
            // );
        };
        self.dragged = slvs_system.dragged;
        self.calculateFaileds = slvs_system.calculateFaileds != 0;
        self.dof = slvs_system.dof;
        self.result = slvs_system.result.into();
    }
}

// Interface to interact with groups
// impl System {
//     pub fn add_group(&mut self) -> Group {
//         self.groups.add(Group::new())
//     }
// }

// Interface to interact with entities
impl System {
    pub fn add_point_3d(&self, group: Group, x: f64, y: f64, z: f64) -> Ref<Entity> {
        let x_param = self.params.push_last(Param::new(group, x));
        let y_param = self.params.push_last(Param::new(group, y));
        let z_param = self.params.push_last(Param::new(group, z));

        self.entities.push_last(Entity::new(
            group.into(),
            EntityType::PointIn3d,
            None,
            [None; 4],
            None,
            None,
            [
                Some((*x_param).into()),
                Some((*y_param).into()),
                Some((*z_param).into()),
                None,
            ],
        ))
    }

    pub fn add_line_3D(&self, group: Group, pt_a: Ref<Entity>, pt_b: Ref<Entity>) -> Ref<Entity> {
        self.entities.push_last(Entity::new(
            group.into(),
            EntityType::LineSegment,
            None,
            [Some((*pt_a).into()), Some((*pt_b).into()), None, None],
            None,
            None,
            [None; 4],
        ))
    }
}

// Interface to interact with constraints
impl System {
    pub fn distance(
        &self,
        group: Group,
        wrkpl: Option<&Entity>,
        distance: f64,
        pt_a: Ref<Entity>,
        pt_b: Ref<Entity>,
    ) -> Ref<Constraint> {
        self.constraints.push_last(Constraint::new(
            group.into(),
            ConstraintType::PtPtDistance, // THERE ARE OTHER TYPES!!!!!
            wrkpl.map(|entity| (*entity).into()),
            distance,
            [Some((*pt_a).into()), Some((*pt_b).into())],
            [None; 4],
            [false, false],
        ))
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

// impl From<binding::Slvs_System> for System {
//     fn from(value: binding::Slvs_System) -> Self {
//         unsafe {
//             Self {
//                 params: RefCell::from(Vec::from_raw_parts(
//                     value.param,
//                     value.params.try_into().unwrap(),
//                     value.params.try_into().unwrap(),
//                 )),
//                 entities: RefCell::from(Vec::from_raw_parts(
//                     value.entity,
//                     value.entities.try_into().unwrap(),
//                     value.entities.try_into().unwrap(),
//                 )),
//                 constraints: RefCell::from(Vec::from_raw_parts(
//                     value.constraint,
//                     value.constraints.try_into().unwrap(),
//                     value.constraints.try_into().unwrap(),
//                 )),
//                 dragged: value.dragged,
//                 calculateFaileds: value.calculateFaileds != 0,
//                 failed: Vec::from_raw_parts(
//                     value.failed,
//                     value.faileds.try_into().unwrap(),
//                     value.faileds.try_into().unwrap(),
//                 ),
//                 dof: value.dof,
//                 result: value.result.into(),
//             }
//         }
//     }
// }

// impl From<System> for binding::Slvs_System {
//     fn from(value: System) -> Self {
//         Self {
//             param: value.params.borrow_mut().as_mut_ptr(),
//             params: value.params.borrow().len() as i32,
//             entity: value.entities.borrow_mut().as_mut_ptr(),
//             entities: value.entities.borrow().len() as i32,
//             constraint: value.constraints.borrow_mut().as_mut_ptr(),
//             constraints: value.entities.borrow().len() as i32,
//             dragged: value.dragged,
//             calculateFaileds: value.calculateFaileds as i32,
//             failed: Vec::with_capacity(value.entities.borrow().len()).as_mut_ptr(),
//             faileds: value.entities.borrow().len() as i32,
//             dof: value.dof,
//             result: value.result as i32,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use std::cell::Ref;

    use crate::{element::group::Group, System};

    #[test]
    fn solve_3d_demo() {
        let mut sys = System::new();
        let g = Group::new();

        let p1 = sys.add_point_3d(g, 10.0, 10.0, 10.0);
        let p2 = sys.add_point_3d(g, 20.0, 20.0, 20.0);
        let l = sys.add_line_3D(g, Ref::clone(&p1), Ref::clone(&p2));
        let constraint = sys.distance(g, None, 30.0, Ref::clone(&p1), Ref::clone(&p1));

        sys.solve(g);
    }
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
