#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{any::Any, sync::atomic::Ordering};

use element::{
    constraint::{Constraint, ConstraintType},
    entity::{Entity, EntityType},
    group::NEXT_GROUP_H,
    param::Param,
    Handle, PushReturn,
};

pub mod element;

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FailReason {
    None,
    Inconsistent = binding::SLVS_RESULT_INCONSISTENT as isize,
    DidntConverge = binding::SLVS_RESULT_DIDNT_CONVERGE as isize,
    TooManyUnknowns = binding::SLVS_RESULT_TOO_MANY_UNKNOWNS as isize,
}

impl TryFrom<i32> for FailReason {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FailReason::None),
            1 => Ok(FailReason::Inconsistent),
            2 => Ok(FailReason::DidntConverge),
            3 => Ok(FailReason::TooManyUnknowns),
            _ => Err("Failure can only take values 0, 1, 2, or 3."),
        }
    }
}

pub struct SolveResult {
    failed: Vec<Handle>,
    dof: i32,
    reason: FailReason,
}

impl SolveResult {
    pub fn new() -> Self {
        Self {
            failed: Vec::<Handle>::new(),
            dof: 0,
            reason: FailReason::None,
        }
    }
}

impl Default for SolveResult {
    fn default() -> Self {
        Self::new()
    }
}

pub struct System {
    groups: Vec<Handle>,
    params: Vec<Param>,
    entities: Vec<Entity>,
    constraints: Vec<Constraint>,
    dragged: [binding::Slvs_hParam; 4],
    calculateFaileds: bool,
    solve_result: SolveResult,
}

impl System {
    pub fn new() -> Self {
        Self {
            groups: Vec::<Handle>::new(),
            params: Vec::<Param>::new(),
            entities: Vec::<Entity>::new(),
            constraints: Vec::<Constraint>::new(),
            dragged: [0; 4],
            calculateFaileds: true,
            solve_result: SolveResult::new(),
        }
    }

    pub fn get<T>(&self, element: Handle) -> Option<Box<dyn Any>> {
        todo!();
        match element {
            Handle::Group(h) => Some(Box::new(Handle::Group(h))),
            Handle::Param(h) => {
                todo!();
                // self.params.iter().find(|&&p| p.h == h).map(|&p| Box::new(p))
            }
            Handle::Entity(h) => todo!(),
            Handle::Constraint(h) => todo!(),
        }
    }

    pub fn set_dragged(&mut self, entity: Handle) {
        if let Handle::Entity(entity_h) = entity {
            if let Some(entity) = self.entities.iter().find(|&&e| e.h == entity_h) {
                self.dragged = entity.param;
            }
        }
    }

    pub fn clear_dragged(&mut self) {
        self.dragged = [0; 4];
    }

    // I want this to return a result.
    pub fn solve(&mut self, group: Handle) {
        let mut slvs_system = binding::Slvs_System {
            param: self.params.as_mut_ptr(),
            params: self.params.len() as i32,
            entity: self.entities.as_mut_ptr(),
            entities: self.entities.len() as i32,
            constraint: self.constraints.as_mut_ptr(),
            constraints: self.constraints.len() as i32,
            dragged: self.dragged,
            calculateFaileds: self.calculateFaileds as i32,
            failed: Vec::with_capacity(self.constraints.len()).as_mut_ptr(),
            faileds: self.constraints.len() as i32,
            dof: self.solve_result.dof,
            result: self.solve_result.reason as i32,
        };

        let failed_constraints: Vec<Handle>;

        unsafe {
            binding::Slvs_Solve(&mut slvs_system, group.into());

            failed_constraints = Vec::from_raw_parts(
                slvs_system.failed,
                slvs_system.faileds.try_into().unwrap(),
                slvs_system.faileds.try_into().unwrap(),
            )
            .into_iter()
            .map(Handle::Constraint)
            .collect();
        };

        self.solve_result = SolveResult {
            failed: failed_constraints,
            dof: slvs_system.dof,
            reason: slvs_system.result.try_into().unwrap(),
        };
    }
}

// Interface to interact with groups
impl System {
    pub fn add_group(&mut self) -> Handle {
        let new_group = Handle::Group(NEXT_GROUP_H.fetch_add(1, Ordering::SeqCst));

        self.groups.push(new_group);
        new_group
    }
}

// Interface to interact with entities
impl System {
    pub fn add_point_3d(
        &mut self,
        group: Handle,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Handle, &'static str> {
        if let Handle::Group(group_h) = group {
            let x_param = self.params.push_return(Param::new(group_h, x));
            let y_param = self.params.push_return(Param::new(group_h, y));
            let z_param = self.params.push_return(Param::new(group_h, z));

            Ok(self.entities.push_return(Entity::new(
                group_h,
                EntityType::PointIn3d,
                None,
                [None; 4],
                None,
                None,
                [
                    Some(x_param.into()),
                    Some(y_param.into()),
                    Some(z_param.into()),
                    None,
                ],
            )))
        } else {
            Err("Invalid Handle type was passed. Should be Group.")
        }
    }

    pub fn add_line_3d(
        &mut self,
        group: Handle,
        pt_a: Handle,
        pt_b: Handle,
    ) -> Result<Handle, &'static str> {
        if let (Handle::Group(group_h), Handle::Entity(pt_a_h), Handle::Entity(pt_b_h)) =
            (group, pt_a, pt_b)
        {
            Ok(self.entities.push_return(Entity::new(
                group_h,
                EntityType::LineSegment,
                None,
                [Some(pt_a_h), Some(pt_b_h), None, None],
                None,
                None,
                [None; 4],
            )))
        } else {
            Err("Invalid Handle types were passed. Should be Group, Entity, Entity")
        }
    }
}

// Interface to interact with constraints
impl System {
    pub fn constrain_distance(
        &mut self,
        group: Handle,
        wrkpl: Option<Handle>,
        distance: f64,
        pt_a: Handle,
        pt_b: Handle,
    ) -> Result<Handle, &'static str> {
        if let (Handle::Group(group_h), Handle::Entity(pt_a_h), Handle::Entity(pt_b_h)) =
            (group, pt_a, pt_b)
        {
            Ok(self.constraints.push_return(Constraint::new(
                group_h,
                ConstraintType::PtPtDistance, // THERE ARE OTHER TYPES!!!!!
                wrkpl.map(|entity| entity.into()),
                distance,
                [Some(pt_a_h), Some(pt_b_h)],
                [None; 4],
                [false, false],
            )))
        } else {
            Err("Invalid Handle types were passed. Should be Group, Entity, Entity")
        }
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FailReason, System};

    #[test]
    fn solve_3d_demo() {
        let mut sys = System::new();
        let g = sys.add_group();

        let p1 = sys
            .add_point_3d(g, 10.0, 10.0, 10.0)
            .expect("Should be Handle::Entity");
        let p2 = sys
            .add_point_3d(g, 20.0, 20.0, 20.0)
            .expect("Should be Handle::Entity");
        sys.add_line_3d(g, p1, p2)
            .expect("Should be Handle::Entity");
        sys.constrain_distance(g, None, 30.0, p1, p2)
            .expect("Should be Handle::Constraint");

        sys.set_dragged(p2);
        sys.solve(g);
        sys.clear_dragged();

        assert_eq!(FailReason::None, sys.solve_result.reason);

        if let FailReason::None = sys.solve_result.reason {
            println!(
                "p1: ({:.3}, {:.3}, {:.3})",
                sys.params[0].val, sys.params[1].val, sys.params[2].val
            );
            println!(
                "p2: ({:.3}, {:.3}, {:.3})",
                sys.params[3].val, sys.params[4].val, sys.params[5].val
            );
        }
    }
}
