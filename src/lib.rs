#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::marker::PhantomData;

use binding::{Slvs_Constraint, Slvs_Entity};
use constraint::{AsConstraint, Constraint, SomeConstraint};
use entity::{AsEntity, Entity, LineSegment, PointIn3d, SomeEntity};
use group::Group;

pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

struct Elements<T> {
    list: Vec<T>,
    next_h: u32,
}

impl<T> Elements<T> {
    fn new() -> Self {
        Self {
            list: Vec::new(),
            next_h: 1,
        }
    }

    fn get_next_h(&mut self) -> u32 {
        let current_h = self.next_h;
        self.next_h += 1;

        current_h
    }
}

impl<T> Default for Elements<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct System {
    groups: Elements<Group>,
    params: Elements<binding::Slvs_Param>,
    entities: Elements<binding::Slvs_Entity>,
    constraints: Elements<binding::Slvs_Constraint>,
    dragged: [binding::Slvs_hParam; 4],
    calculate_faileds: bool,
}

impl System {
    pub fn new() -> Self {
        Self {
            groups: Elements::default(),
            params: Elements::default(),
            entities: Elements::default(),
            constraints: Elements::default(),
            dragged: [0; 4],
            calculate_faileds: true,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Adding Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn add_group(&mut self) -> Group {
        let new_group = Group(self.groups.get_next_h());

        self.groups.list.push(new_group);
        self.groups.list.last().cloned().unwrap()
    }

    pub fn add_entity<T>(&mut self, group: Group, entity: T) -> Result<Entity<T>, &'static str>
    where
        T: AsEntity,
    {
        let new_entity = binding::Slvs_Entity {
            h: self.entities.get_next_h(),
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

        self.entities.list.push(new_entity);
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
        let [point_a, point_b] = constraint.point();
        let [entity_a, entity_b, entity_c, entity_d] = constraint.entity();
        let [other, other_2] = constraint.other();

        let new_constraint = binding::Slvs_Constraint {
            h: self.constraints.get_next_h(),
            group: group.into(),
            type_: constraint.type_() as _,
            wrkpl: constraint.workplane().unwrap_or(0), // TODO: check that entity exists and is the correct type
            valA: constraint.val(),
            ptA: point_a.unwrap_or(0),      // TODO: ditto
            ptB: point_b.unwrap_or(0),      // TODO: ditto
            entityA: entity_a.unwrap_or(0), // TODO: ditto
            entityB: entity_b.unwrap_or(0), // TODO: ditto
            entityC: entity_c.unwrap_or(0), // TODO: ditto
            entityD: entity_d.unwrap_or(0), // TODO: ditto
            other: other as _,
            other2: other_2 as _,
        };

        self.constraints.list.push(new_constraint);
        Ok(Constraint {
            handle: new_constraint.h,
            phantom: PhantomData,
        })
    }

    // Private as user has no reason to create bare param without linking to an entity.
    fn add_param(&mut self, group: Group, val: f64) -> binding::Slvs_hParam {
        let new_param = binding::Slvs_Param {
            h: self.params.get_next_h(),
            group: group.into(),
            val,
        };

        self.params.list.push(new_param);
        self.params.list.last().unwrap().h
    }
}

////////////////////////////////////////////////////////////////////////////////
// Getting Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn get_entity(&self, entity: SomeEntity) -> Option<Box<dyn AsEntity>> {
        self.h_to_slvs_entity(entity.into())
            .map(|slvs_entity| match entity {
                SomeEntity::PointIn3d(_) => Box::new(PointIn3d {
                    x: self.h_to_slvs_param(slvs_entity.param[0]).unwrap().val,
                    y: self.h_to_slvs_param(slvs_entity.param[1]).unwrap().val,
                    z: self.h_to_slvs_param(slvs_entity.param[2]).unwrap().val,
                }) as Box<dyn AsEntity>,
                SomeEntity::LineSegment(_) => Box::new(LineSegment {
                    point_a: Entity::new(slvs_entity.point[0]),
                    point_b: Entity::new(slvs_entity.point[1]),
                }) as Box<dyn AsEntity>,
            })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Solving the system
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn set_dragged(&mut self, entity: Entity<impl AsEntity>) {
        if let Some(slvs_entity) = self.h_to_slvs_entity(entity.into()) {
            self.dragged = slvs_entity.param;
        }
    }

    pub fn clear_dragged(&mut self) {
        self.dragged = [0; 4];
    }

    pub fn solve(&mut self, group: Group) -> Result<SolveOkay, SolveFail> {
        let mut failed_handles: Vec<binding::Slvs_hConstraint> =
            vec![0; self.constraints.list.len()];

        let mut slvs_system = binding::Slvs_System {
            param: self.params.list.as_mut_ptr(),
            params: self.params.list.len() as _,
            entity: self.entities.list.as_mut_ptr(),
            entities: self.entities.list.len() as _,
            constraint: self.constraints.list.as_mut_ptr(),
            constraints: self.constraints.list.len() as _,
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

        match FailReason::try_from(slvs_system.result) {
            Ok(fail_reason) => Err(SolveFail {
                dof: slvs_system.dof,
                reason: fail_reason,
                failed_constraints: failed_handles
                    .into_iter()
                    .map(|h| self.h_to_some_constraint(h).unwrap())
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

pub struct SolveFail {
    pub dof: i32,
    pub reason: FailReason,
    pub failed_constraints: Vec<SomeConstraint>,
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

////////////////////////////////////////////////////////////////////////////////
// Internal methods for Slvs_Handles -> other stuff
////////////////////////////////////////////////////////////////////////////////

impl System {
    fn h_to_slvs_param(&self, h: binding::Slvs_hParam) -> Option<&binding::Slvs_Param> {
        self.params
            .list
            .binary_search_by_key(&h, |&binding::Slvs_Param { h, .. }| h)
            .map_or(None, |ix| Some(&self.params.list[ix]))
    }

    fn h_to_slvs_entity(&self, h: binding::Slvs_hEntity) -> Option<&binding::Slvs_Entity> {
        self.entities
            .list
            .binary_search_by_key(&h, |&binding::Slvs_Entity { h, .. }| h)
            .map_or(None, |ix| Some(&self.entities.list[ix]))
    }

    fn h_to_some_entity(&self, h: binding::Slvs_hEntity) -> Option<SomeEntity> {
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
            .list
            .binary_search_by_key(&h, |&binding::Slvs_Constraint { h, .. }| h)
            .map_or(None, |ix| Some(&self.constraints.list[ix]))
    }

    fn h_to_some_constraint(&self, h: binding::Slvs_hConstraint) -> Option<SomeConstraint> {
        self.h_to_slvs_constraint(h)
            .map(|Slvs_Constraint { h, type_, .. }| match *type_ as _ {
                binding::SLVS_C_PT_PT_DISTANCE => SomeConstraint::PtPtDistance(Constraint::new(*h)),
                _ => panic!("Unknown constraint type: {}", type_),
            })
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

        sys.add_entity(
            g,
            LineSegment {
                point_a: p1,
                point_b: p2,
            },
        )
        .expect("line segment created");

        let target_dist = 30.0;
        sys.add_constraint(
            g,
            PtPtDistance::_3d {
                val: target_dist,
                point_a: p1,
                point_b: p2,
            },
        )
        .expect("distance constraint added");

        sys.set_dragged(p2);
        let solve_result = sys.solve(g);
        sys.clear_dragged();

        if solve_result.is_ok() {
            sys.params
                .list
                .iter()
                .for_each(|param| println!("Handle {}: {:.3}", param.h, param.val));

            let dist = ((sys.params.list[0].val - sys.params.list[3].val).powi(2)
                + (sys.params.list[1].val - sys.params.list[4].val).powi(2)
                + (sys.params.list[2].val - sys.params.list[5].val).powi(2))
            .sqrt();

            assert!((target_dist - dist).abs() < SOLVE_TOLERANCE);
        }
    }
}
