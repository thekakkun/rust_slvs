#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use constraint::Constraint;
use entity::{AsEntity, Entity};
use group::Group;
use param::Param;

pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SolveResult {
    None = binding::SLVS_RESULT_OKAY as _,
    Inconsistent = binding::SLVS_RESULT_INCONSISTENT as _,
    DidntConverge = binding::SLVS_RESULT_DIDNT_CONVERGE as _,
    TooManyUnknowns = binding::SLVS_RESULT_TOO_MANY_UNKNOWNS as _,
}

impl TryFrom<i32> for SolveResult {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, &'static str> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Inconsistent),
            2 => Ok(Self::DidntConverge),
            3 => Ok(Self::TooManyUnknowns),
            _ => Err("Failure can only take values 0, 1, 2, or 3."),
        }
    }
}

pub struct System<'a> {
    groups: Vec<Group>,
    next_group_h: u32,
    params: Vec<Param>,
    next_param_h: u32,
    entities: Vec<Entity>,
    next_entity_h: u32,
    constraints: Vec<Constraint>,
    next_constraint_h: u32,
    dragged: [binding::Slvs_hParam; 4],
    calculateFaileds: bool,
    failed: Vec<&'a Constraint>,
    dof: i32,
    result: SolveResult,
}

impl System<'_> {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            next_group_h: 1,
            params: Vec::new(),
            next_param_h: 1,
            entities: Vec::new(),
            next_entity_h: 1,
            constraints: Vec::new(),
            next_constraint_h: 1,
            dragged: [0; 4],
            calculateFaileds: true,
            failed: Vec::new(),
            dof: 0,
            result: SolveResult::None,
        }
    }

    pub fn solve(&mut self, group: &Group) {
        let mut failed_handles: Vec<binding::Slvs_hConstraint> = vec![0; self.constraints.len()];

        let mut slvs_system = binding::Slvs_System {
            param: self.params.as_mut_ptr(),
            params: self.params.len() as _,
            entity: self.entities.as_mut_ptr(),
            entities: self.entities.len() as _,
            constraint: self.constraints.as_mut_ptr(),
            constraints: self.constraints.len() as _,
            dragged: self.dragged,
            calculateFaileds: self.calculateFaileds as _,
            failed: failed_handles.as_mut_ptr(),
            faileds: failed_handles.len() as _,
            dof: self.dof,
            result: self.result as _,
        };

        unsafe {
            binding::Slvs_Solve(&mut slvs_system, (*group).into());

            failed_handles = Vec::from_raw_parts(
                slvs_system.failed,
                slvs_system.faileds.try_into().unwrap(),
                slvs_system.faileds.try_into().unwrap(),
            )
        };

        self.failed = failed_handles
            .into_iter()
            .map(|h: binding::Slvs_hConstraint| self.get_constraint(h).unwrap())
            .collect();
        self.dof = slvs_system.dof;
        self.result = slvs_system.result.try_into().unwrap();
    }
}

// Methods for interfacing with group
impl System<'_> {
    pub fn add_group(&mut self) -> &Group {
        let new_group = Group(self.next_group_h);
        self.next_group_h += 1;

        let groups = &mut self.groups;

        groups.push(new_group);
        groups.last().unwrap()
    }
}

// Methods for interfacing with params
impl System<'_> {
    fn add_param(&mut self, group: &Group, val: f64) -> binding::Slvs_hParam {
        let new_param = Param {
            h: self.next_param_h,
            group: (*group).into(),
            val,
        };
        self.next_param_h += 1;

        self.params.push(new_param);
        self.params.last().unwrap().h
    }

    fn get_param(&self, h: binding::Slvs_hParam) -> Option<&Param> {
        self.params.iter().find(|&param| param.h == h)
    }
}

// Methods for interfacing with entities
impl System<'_> {
    pub fn add_entity(
        &mut self,
        group: &Group,
        entity: impl AsEntity,
    ) -> Result<&Entity, &'static str> {
        let params: [u32; 4] = entity
            .param_vals()
            .iter()
            .map(|&param| {
                if let Some(val) = param {
                    self.add_param(group, val)
                } else {
                    0
                }
            })
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        let new_entity = Entity {
            h: self.next_entity_h,
            group: (*group).into(),
            type_: entity.type_() as _,
            wrkpl: entity.wrkpl().unwrap_or(0), // TODO: requires check that entity with handle exists
            point: entity.point().map(|p| p.unwrap_or(0)), // TODO: ditto
            normal: entity.normal().unwrap_or(0), // TODO: ditto
            distance: entity.distance().unwrap_or(0), // TODO: ditto
            param: params,
        };
        self.next_entity_h += 1;

        self.entities.push(new_entity);
        Ok(self.entities.last().unwrap())
    }

    fn get_entity(&self, h: binding::Slvs_hEntity) -> Option<&Entity> {
        self.entities.iter().find(|&entity| entity.h == h)
    }
}

// Methods for interfacing with constraints
impl System<'_> {
    fn get_constraint(&self, h: binding::Slvs_hConstraint) -> Option<&Constraint> {
        self.constraints
            .iter()
            .find(|&constraint| constraint.h == h)
    }
}

impl Default for System<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // use crate::{element::entity::PointIn3d, System};

    use crate::{entity::PointIn3d, System};

    #[test]
    fn solve_3d_demo() {
        let mut sys = System::new();
        let g = sys.add_group();
        let p1 = sys.add_entity(
            g,
            PointIn3d {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
        );
        let p2 = sys.add_entity(
            g,
            PointIn3d {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
        );

        // let p1 = sys
        //     .add_point_3d(g, 10.0, 10.0, 10.0)
        //     .expect("Should be Handle::Entity");
        // let p2 = sys
        //     .add_point_3d(g, 20.0, 20.0, 20.0)
        //     .expect("Should be Handle::Entity");
        // sys.add_line_3d(g, p1, p2)
        //     .expect("Should be Handle::Entity");
        // sys.constrain_distance(g, None, 30.0, p1, p2)
        //     .expect("Should be Handle::Constraint");

        // sys.set_dragged(p2);
        // sys.solve(g);
        // sys.clear_dragged();

        // assert_eq!(FailReason::None, sys.solve_result.reason);

        // if let FailReason::None = sys.solve_result.reason {
        //     println!(
        //         "p1: ({:.3}, {:.3}, {:.3})",
        //         sys.params[0].val, sys.params[1].val, sys.params[2].val
        //     );
        //     println!(
        //         "p2: ({:.3}, {:.3}, {:.3})",
        //         sys.params[3].val, sys.params[4].val, sys.params[5].val
        //     );
        // }
    }
}

// impl SolveResult {
//     pub fn new() -> Self {
//         Self {
//             failed: Vec::<Handle>::new(),
//             dof: 0,
//             reason: FailReason::None,
//         }
//     }
// }

// impl Default for SolveResult {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// pub struct System {
//     groups: Vec<Handle>,
//     params: Vec<Param>,
//     entities: Vec<Entity>,
//     constraints: Vec<Constraint>,
//     dragged: [binding::Slvs_hParam; 4],
//     calculateFaileds: bool,
//     solve_result: SolveResult,
// }

// impl System {
//     pub fn new() -> Self {
//         Self {
//             groups: Vec::<Handle>::new(),
//             params: Vec::<Param>::new(),
//             entities: Vec::<Entity>::new(),
//             constraints: Vec::<Constraint>::new(),
//             dragged: [0; 4],
//             calculateFaileds: true,
//             solve_result: SolveResult::new(),
//         }
//     }

//     pub fn get<T>(&self, element: Handle) -> Option<Box<dyn Any>> {
//         todo!();
//         match element {
//             Handle::Group(h) => Some(Box::new(Handle::Group(h))),
//             Handle::Param(h) => {
//                 todo!();
//                 // self.params.iter().find(|&&p| p.h == h).map(|&p| Box::new(p))
//             }
//             Handle::Entity(h) => todo!(),
//             Handle::Constraint(h) => todo!(),
//         }
//     }

//     pub fn set_dragged(&mut self, entity: Handle) {
//         if let Handle::Entity(entity_h) = entity {
//             if let Some(entity) = self.entities.iter().find(|&&e| e.h == entity_h) {
//                 self.dragged = entity.param;
//             }
//         }
//     }

//     pub fn clear_dragged(&mut self) {
//         self.dragged = [0; 4];
//     }

//     pub fn solve(&mut self, group: Handle) {
//         let mut slvs_system = binding::Slvs_System {
//             param: self.params.as_mut_ptr(),
//             params: self.params.len() as i32,
//             entity: self.entities.as_mut_ptr(),
//             entities: self.entities.len() as i32,
//             constraint: self.constraints.as_mut_ptr(),
//             constraints: self.constraints.len() as i32,
//             dragged: self.dragged,
//             calculateFaileds: self.calculateFaileds as i32,
//             failed: Vec::with_capacity(self.constraints.len()).as_mut_ptr(),
//             faileds: self.constraints.len() as i32,
//             dof: self.solve_result.dof,
//             result: self.solve_result.reason as i32,
//         };

//         let failed_constraints: Vec<Handle>;

//         unsafe {
//             binding::Slvs_Solve(&mut slvs_system, group.into());

//             failed_constraints = Vec::from_raw_parts(
//                 slvs_system.failed,
//                 slvs_system.faileds.try_into().unwrap(),
//                 slvs_system.faileds.try_into().unwrap(),
//             )
//             .into_iter()
//             .map(Handle::Constraint)
//             .collect();
//         };

//         self.solve_result = SolveResult {
//             failed: failed_constraints,
//             dof: slvs_system.dof,
//             reason: slvs_system.result.try_into().unwrap(),
//         };
//     }
// }

// // Interface to interact with groups
// impl System {
//     pub fn add_group(&mut self) -> Handle {
//         let new_group = Handle::Group(NEXT_GROUP_H.fetch_add(1, Ordering::SeqCst));

//         self.groups.push(new_group);
//         new_group
//     }
// }

// // Interface to interact with entities
// impl System {
//     pub fn add_point_3d(
//         &mut self,
//         group: Handle,
//         x: f64,
//         y: f64,
//         z: f64,
//     ) -> Result<Handle, &'static str> {
//         if let Handle::Group(group_h) = group {
//             let x_param = self.params.push_return(Param::new(group_h, x));
//             let y_param = self.params.push_return(Param::new(group_h, y));
//             let z_param = self.params.push_return(Param::new(group_h, z));

//             Ok(self.entities.push_return(Entity::new(
//                 group_h,
//                 EntityType::PointIn3d,
//                 None,
//                 [None; 4],
//                 None,
//                 None,
//                 [
//                     Some(x_param.into()),
//                     Some(y_param.into()),
//                     Some(z_param.into()),
//                     None,
//                 ],
//             )))
//         } else {
//             Err("Invalid Handle type was passed. Should be Group.")
//         }
//     }

//     pub fn add_line_3d(
//         &mut self,
//         group: Handle,
//         pt_a: Handle,
//         pt_b: Handle,
//     ) -> Result<Handle, &'static str> {
//         if let (Handle::Group(group_h), Handle::Entity(pt_a_h), Handle::Entity(pt_b_h)) =
//             (group, pt_a, pt_b)
//         {
//             Ok(self.entities.push_return(Entity::new(
//                 group_h,
//                 EntityType::LineSegment,
//                 None,
//                 [Some(pt_a_h), Some(pt_b_h), None, None],
//                 None,
//                 None,
//                 [None; 4],
//             )))
//         } else {
//             Err("Invalid Handle types were passed. Should be Group, Entity, Entity")
//         }
//     }
// }

// // Interface to interact with constraints
// impl System {
//     pub fn constrain_distance(
//         &mut self,
//         group: Handle,
//         wrkpl: Option<Handle>,
//         distance: f64,
//         pt_a: Handle,
//         pt_b: Handle,
//     ) -> Result<Handle, &'static str> {
//         if let (Handle::Group(group_h), Handle::Entity(pt_a_h), Handle::Entity(pt_b_h)) =
//             (group, pt_a, pt_b)
//         {
//             Ok(self.constraints.push_return(Constraint::new(
//                 group_h,
//                 ConstraintType::PtPtDistance, // THERE ARE OTHER TYPES!!!!!
//                 wrkpl.map(|entity| entity.into()),
//                 distance,
//                 [Some(pt_a_h), Some(pt_b_h)],
//                 [None; 4],
//                 [false, false],
//             )))
//         } else {
//             Err("Invalid Handle types were passed. Should be Group, Entity, Entity")
//         }
//     }
// }

// impl Default for System {
//     fn default() -> Self {
//         Self::new()
//     }
// }
