#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::marker::PhantomData;

use binding::Slvs_Entity;
use constraint::{AsConstraint, Constraint, PtPtDistance};
use entity::{AsEntity, Entity, LineSegment, PointIn3d};
use group::Group;

pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct System {
    groups: Vec<Group>,
    next_group_h: u32,
    pub params: Vec<binding::Slvs_Param>, // public for testing purposes. implement a getter.
    next_param_h: u32,
    entities: Vec<binding::Slvs_Entity>,
    next_entity_h: u32,
    constraints: Vec<binding::Slvs_Constraint>,
    next_constraint_h: u32,
    dragged: [binding::Slvs_hParam; 4],
    calculate_faileds: bool,
}

impl System {
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
            calculate_faileds: true,
        }
    }
}

// Adding elements
impl System {
    pub fn add_group(&mut self) -> Group {
        let new_group = Group(self.next_group_h);
        self.next_group_h += 1;

        self.groups.push(new_group);
        self.groups.last().cloned().unwrap()
    }

    pub fn add_entity<T>(&mut self, group: Group, entity: T) -> Result<Entity<T>, &'static str>
    where
        T: AsEntity,
    {
        let new_entity = binding::Slvs_Entity {
            h: self.next_entity_h,
            group: group.into(),
            type_: entity.type_() as _,
            wrkpl: entity.workplane().unwrap_or(0), // TODO: check that entity exists and is the correct type
            point: entity.point().map(|p| p.unwrap_or(0)), // TODO: ditto
            normal: entity.normal().unwrap_or(0),   // TODO: ditto
            distance: entity.distance().unwrap_or(0), // TODO: ditto
            param: entity
                .param_vals()
                .map(|opt_val| opt_val.map_or(0, |v| self.add_param(group, v))),
        };
        self.next_entity_h += 1;

        self.entities.push(new_entity);
        Ok(Entity {
            handle: new_entity.h,
            phantom: PhantomData,
        })
    }

    pub fn add_constraint<T>(
        &mut self,
        group: Group,
        constraint: T,
    ) -> Result<Constraint<T>, &'static str>
    where
        T: AsConstraint,
    {
        let [pt_a, pt_b] = constraint.point();
        let [entity_a, entity_b, entity_c, entity_d] = constraint.entity();
        let [other, other_2] = constraint.other();

        let new_constraint = binding::Slvs_Constraint {
            h: self.next_constraint_h,
            group: group.into(),
            type_: constraint.type_() as _,
            wrkpl: constraint.workplane().unwrap_or(0), // TODO: check that entity exists and is the correct type
            valA: constraint.val(),
            ptA: pt_a.unwrap_or(0),         // TODO: ditto
            ptB: pt_b.unwrap_or(0),         // TODO: ditto
            entityA: entity_a.unwrap_or(0), // TODO: ditto
            entityB: entity_b.unwrap_or(0), // TODO: ditto
            entityC: entity_c.unwrap_or(0), // TODO: ditto
            entityD: entity_d.unwrap_or(0), // TODO: ditto
            other: other as _,
            other2: other_2 as _,
        };
        self.next_constraint_h += 1;

        self.constraints.push(new_constraint);
        Ok(Constraint {
            handle: new_constraint.h,
            phantom: PhantomData,
        })
    }

    // Private as user has no reason to create bare param without linking to an entity.
    fn add_param(&mut self, group: Group, val: f64) -> binding::Slvs_hParam {
        let new_param = binding::Slvs_Param {
            h: self.next_param_h,
            group: group.into(),
            val,
        };
        self.next_param_h += 1;

        self.params.push(new_param);
        self.params.last().unwrap().h
    }
}

// Solving the system
impl System {
    pub fn set_dragged(&mut self, entity: Entity<impl AsEntity>) {
        if let Some(slvs_entity) = self.h_to_slvs_entity(entity.into()) {
            self.dragged = slvs_entity.param;
        }
    }

    pub fn clear_dragged(&mut self) {
        self.dragged = [0; 4];
    }

    pub fn solve(&mut self, group: Group) -> Result<SolveOkay, SolveFail<impl AsConstraint>> {
        let mut failed_handles: Vec<binding::Slvs_hConstraint> = vec![0; self.constraints.len()];

        let mut slvs_system = binding::Slvs_System {
            param: self.params.as_mut_ptr(),
            params: self.params.len() as _,
            entity: self.entities.as_mut_ptr(),
            entities: self.entities.len() as _,
            constraint: self.constraints.as_mut_ptr(),
            constraints: self.constraints.len() as _,
            dragged: self.dragged,
            calculateFaileds: self.calculate_faileds as _,
            failed: failed_handles.as_mut_ptr(),
            faileds: failed_handles.len() as _,
            dof: 0,
            result: 0,
        };

        unsafe {
            binding::Slvs_Solve(&mut slvs_system, group.into());

            failed_handles = Vec::from_raw_parts(
                slvs_system.failed,
                slvs_system.faileds.try_into().unwrap(),
                slvs_system.faileds.try_into().unwrap(),
            )
        };

        // let foo: Vec<_> = failed_handles
        //     .into_iter()
        //     .map(|h| self.h_to_constraint(h).unwrap())
        //     .collect();

        match FailReason::try_from(slvs_system.result) {
            Ok(fail_reason) => Err(SolveFail {
                dof: slvs_system.dof,
                reason: fail_reason,
                failed_constraints: failed_handles
                    .into_iter()
                    .map(|h| self.h_to_constraint(h).unwrap())
                    .collect(),
            }),
            Err(_) => Ok(SolveOkay {
                dof: slvs_system.dof,
            }),
        }
    }
}

pub struct SolveOkay {
    pub dof: i32,
}

pub struct SolveFail<T: AsConstraint + ?Sized> {
    pub dof: i32,
    pub reason: FailReason,
    pub failed_constraints: Vec<Constraint<T>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FailReason {
    Inconsistent = binding::SLVS_RESULT_INCONSISTENT as _,
    DidntConverge = binding::SLVS_RESULT_DIDNT_CONVERGE as _,
    TooManyUnknowns = binding::SLVS_RESULT_TOO_MANY_UNKNOWNS as _,
}

impl TryFrom<i32> for FailReason {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, &'static str> {
        match value {
            1 => Ok(Self::Inconsistent),
            2 => Ok(Self::DidntConverge),
            3 => Ok(Self::TooManyUnknowns),
            _ => Err("Result must be of values 1, 2, or 3."),
        }
    }
}

// Internal methods for Slvs_Handles -> other stuff
impl System {
    fn h_to_slvs_param(&self, h: binding::Slvs_hParam) -> Option<&binding::Slvs_Param> {
        self.params
            .binary_search_by_key(&h, |&binding::Slvs_Param { h, .. }| h)
            .map_or(None, |ix| Some(&self.params[ix]))
    }

    fn h_to_slvs_entity(&self, h: binding::Slvs_hEntity) -> Option<&binding::Slvs_Entity> {
        self.entities
            .binary_search_by_key(&h, |&binding::Slvs_Entity { h, .. }| h)
            .map_or(None, |ix| Some(&self.entities[ix]))
    }

    pub fn h_to_entity(&self, h: binding::Slvs_hEntity) -> Option<SomeEntity> {
        self.h_to_slvs_entity(h)
            .map(|Slvs_Entity { h, type_, .. }| match *type_ as _ {
                binding::SLVS_E_POINT_IN_3D => SomeEntity::PointIn3d(Entity::new(*h)),
                binding::SLVS_E_POINT_IN_2D => todo!(),
                binding::SLVS_E_NORMAL_IN_3D => todo!(),
                binding::SLVS_E_NORMAL_IN_2D => todo!(),
                binding::SLVS_E_DISTANCE => todo!(),
                binding::SLVS_E_WORKPLANE => todo!(),
                binding::SLVS_E_LINE_SEGMENT => SomeEntity::LineSegment(Entity::new(*h)),
                binding::SLVS_E_CUBIC => todo!(),
                binding::SLVS_E_CIRCLE => todo!(),
                binding::SLVS_E_ARC_OF_CIRCLE => todo!(),
                _ => panic!("Unknown entity type: {}", type_),
            })
    }

    fn h_to_slvs_constraint(
        &self,
        h: binding::Slvs_hConstraint,
    ) -> Option<&binding::Slvs_Constraint> {
        self.constraints
            .binary_search_by_key(&h, |&binding::Slvs_Constraint { h, .. }| h)
            .map_or(None, |ix| Some(&self.constraints[ix]))
    }

    fn h_to_constraint(
        &self,
        h: binding::Slvs_hConstraint,
    ) -> Option<Constraint<impl AsConstraint>> {
        if let Some(slvs_entity) = self.h_to_slvs_entity(h) {
            match slvs_entity.type_ as _ {
                binding::SLVS_C_PT_PT_DISTANCE => Some(Constraint::<PtPtDistance> {
                    handle: slvs_entity.h,
                    phantom: PhantomData,
                }),
            }
        } else {
            None
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
    use crate::{
        constraint::PtPtDistance,
        entity::{LineSegment, PointIn3d},
        System,
    };

    const SOLVE_TOLERANCE: f64 = 1e-8;

    #[test]
    fn solve_3d_demo() {
        let mut sys = System::new();
        let g = sys.add_group();
        let p1 = sys
            .add_entity(
                g,
                PointIn3d {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                },
            )
            .expect("p1 created");
        let p2 = sys
            .add_entity(
                g,
                PointIn3d {
                    x: 20.0,
                    y: 20.0,
                    z: 20.0,
                },
            )
            .expect("p2 created");

        sys.add_entity(g, LineSegment { pt_a: p1, pt_b: p2 })
            .expect("line segment created");
        let target_dist = 30.0;
        sys.add_constraint(
            g,
            PtPtDistance::_3d {
                val: target_dist,
                pt_a: p1,
                pt_b: p2,
            },
        )
        .expect("distance constraint added");

        sys.set_dragged(p2);
        let solve_result = sys.solve(g);
        sys.clear_dragged();

        if solve_result.is_ok() {
            sys.params
                .iter()
                .for_each(|param| println!("Handle {}: {:.3}", param.h, param.val));

            let dist = ((sys.params[0].val - sys.params[3].val).powi(2)
                + (sys.params[1].val - sys.params[4].val).powi(2)
                + (sys.params[2].val - sys.params[5].val).powi(2))
            .sqrt();

            assert!((target_dist - dist).abs() < SOLVE_TOLERANCE);
        }
    }
}
