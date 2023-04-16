use std::any::Any;
use std::{iter::zip, marker::PhantomData};

use bindings::{Slvs_Constraint, Slvs_hConstraint};
use bindings::{
    Slvs_Entity, Slvs_hEntity, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE, SLVS_E_CUBIC, SLVS_E_DISTANCE,
    SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D, SLVS_E_POINT_IN_2D,
    SLVS_E_POINT_IN_3D, SLVS_E_WORKPLANE,
};
use bindings::{Slvs_Param, Slvs_hParam};
use bindings::{
    Slvs_Solve, Slvs_System, SLVS_RESULT_DIDNT_CONVERGE, SLVS_RESULT_INCONSISTENT,
    SLVS_RESULT_TOO_MANY_UNKNOWNS,
};
use constraint::{AsConstraint, Constraint, SomeConstraint};
use entity::{AsEntity, Entity, LineSegment, PointIn3d};

mod bindings;
pub mod constraint;
pub mod entity;

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

#[derive(Clone, Copy)]
pub struct Group(u32);

pub struct System {
    groups: Elements<Group>,
    params: Elements<Slvs_Param>,
    entities: Elements<Slvs_Entity>,
    constraints: Elements<Slvs_Constraint>,
    dragged: [Slvs_hParam; 4],
    calculate_faileds: bool,
}

impl System {
    pub fn new() -> Self {
        Self {
            groups: Elements::new(),
            params: Elements::new(),
            entities: Elements::new(),
            constraints: Elements::new(),
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

    pub fn add_entity<T: AsEntity>(
        &mut self,
        group: Group,
        entity_data: T,
    ) -> Result<Entity<T>, &'static str> {
        let validated_entity = self.validate_entity(entity_data)?;

        let mut new_entity = Slvs_Entity::new(
            self.entities.get_next_h(),
            group.into(),
            validated_entity.type_(),
        );

        if let Some(workplane) = validated_entity.workplane() {
            new_entity.workplane(workplane);
        }

        if let Some(mut points) = validated_entity.points() {
            points.resize(4, 0);
            new_entity.point(points.try_into().unwrap());
        }

        if let Some(normal) = validated_entity.normal() {
            new_entity.normal(normal);
        }

        if let Some(distance) = validated_entity.distance() {
            new_entity.distance(distance);
        }

        if let Some(param_vals) = validated_entity.param_vals() {
            let mut param_h: Vec<_> = param_vals
                .iter()
                .map(|val| self.add_param(group, *val))
                .collect();
            param_h.resize(4, 0);

            new_entity.param(param_h.try_into().unwrap())
        }

        self.entities.list.push(new_entity);
        Ok(Entity {
            handle: new_entity.h,
            phantom: PhantomData,
        })
    }

    pub fn add_constraint<T: AsConstraint>(
        &mut self,
        group: Group,
        constraint: T,
    ) -> Result<Constraint<T>, &'static str> {
        let [point_a, point_b] = constraint.point();
        let [entity_a, entity_b, entity_c, entity_d] = constraint.entity();
        let [other, other_2] = constraint.other();

        let new_constraint = Slvs_Constraint {
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
    fn add_param(&mut self, group: Group, val: f64) -> Slvs_hParam {
        let new_param = Slvs_Param {
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
    pub fn get_entity_data<T>(&self, entity: Entity<T>) -> Option<T>
    where
        T: AsEntity + Copy + 'static,
        Entity<T>: Into<Slvs_hEntity>,
    {
        self.h_to_slvs_entity(entity.into()).map(|slvs_entity| {
            let some_entity: Box<dyn Any> = match slvs_entity.type_ as _ {
                SLVS_E_POINT_IN_3D => Box::new(PointIn3d {
                    x: self.h_to_slvs_param(slvs_entity.param[0]).unwrap().val,
                    y: self.h_to_slvs_param(slvs_entity.param[1]).unwrap().val,
                    z: self.h_to_slvs_param(slvs_entity.param[2]).unwrap().val,
                }),
                SLVS_E_POINT_IN_2D => todo!(),
                SLVS_E_NORMAL_IN_3D => todo!(),
                SLVS_E_NORMAL_IN_2D => todo!(),
                SLVS_E_DISTANCE => todo!(),
                SLVS_E_WORKPLANE => todo!(),
                SLVS_E_LINE_SEGMENT => Box::new(LineSegment {
                    point_a: Entity::new(slvs_entity.point[0]),
                    point_b: Entity::new(slvs_entity.point[1]),
                }),
                SLVS_E_CUBIC => todo!(),
                SLVS_E_CIRCLE => todo!(),
                SLVS_E_ARC_OF_CIRCLE => todo!(),
                _ => panic!("Unknown entity type: {}", slvs_entity.type_),
            };

            *some_entity.downcast::<T>().unwrap()
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Updating Elements
///////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn update_entity<T, F>(&mut self, entity: Entity<T>, f: F) -> Result<T, &'static str>
    where
        T: AsEntity + Copy + 'static,
        Entity<T>: Into<Slvs_hEntity>,
        F: FnOnce(&mut T),
    {
        if let Some(mut entity_data) = self.get_entity_data(entity) {
            f(&mut entity_data);

            let validated_entity_data = self.validate_entity(entity_data)?;

            // scoped to allow another mut self for the params.
            let param_h = {
                // safe to unwrap() we've already confirmed `entity` exists in the `if let` above
                let mut slvs_entity = self.h_to_mut_slvs_entity(entity.into()).unwrap();

                slvs_entity.wrkpl = validated_entity_data.workplane().unwrap_or(0);
                slvs_entity.point =
                    validated_entity_data
                        .points()
                        .map_or([0, 0, 0, 0], |mut points| {
                            points.resize(4, 0);
                            points.try_into().unwrap()
                        });
                slvs_entity.normal = validated_entity_data.normal().unwrap_or(0);
                slvs_entity.distance = validated_entity_data.distance().unwrap_or(0);

                slvs_entity.param
            };

            if let Some(param_vals) = validated_entity_data.param_vals() {
                let h_val_iter = zip(param_h, param_vals);
                for (h, val) in h_val_iter {
                    self.update_param(h, val)?;
                }
            }

            Ok(validated_entity_data)
        } else {
            Err("Entity not found")
        }
    }

    fn update_param(&mut self, h: Slvs_hParam, val: f64) -> Result<(), &'static str> {
        if let Some(param) = self.h_to_mut_slvs_param(h) {
            param.val = val;

            Ok(())
        } else {
            Err("Param not found")
        }
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
        let mut failed_handles: Vec<Slvs_hConstraint> = vec![0; self.constraints.list.len()];

        let mut slvs_system = Slvs_System {
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
            Slvs_Solve(&mut slvs_system, group.into());

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
    Inconsistent,
    DidntConverge,
    TooManyUnknowns,
}

impl TryFrom<i32> for FailReason {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, &'static str> {
        match value as _ {
            SLVS_RESULT_INCONSISTENT => Ok(Self::Inconsistent),
            SLVS_RESULT_DIDNT_CONVERGE => Ok(Self::DidntConverge),
            SLVS_RESULT_TOO_MANY_UNKNOWNS => Ok(Self::TooManyUnknowns),
            _ => Err("Result must be of values 1, 2, or 3."),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Internal methods for Slvs_Handles -> other stuff
////////////////////////////////////////////////////////////////////////////////

impl System {
    fn h_to_slvs_param(&self, h: Slvs_hParam) -> Option<&Slvs_Param> {
        self.params
            .list
            .binary_search_by_key(&h, |&Slvs_Param { h, .. }| h)
            .map_or(None, |ix| Some(&self.params.list[ix]))
    }

    fn h_to_mut_slvs_param(&mut self, h: Slvs_hParam) -> Option<&mut Slvs_Param> {
        self.params
            .list
            .binary_search_by_key(&h, |&Slvs_Param { h, .. }| h)
            .map_or(None, |ix| Some(&mut self.params.list[ix]))
    }

    fn validate_entity<T: AsEntity>(&self, entity: T) -> Result<T, &'static str> {
        if let Some(workplane) = entity.workplane() {
            if !self.entity_exists(workplane) {
                return Err("Specified workplane does not exist.");
            }
        } else if let Some(points) = entity.points() {
            if !points
                .iter()
                .map(|&point| self.entity_exists(point))
                .all(|x| x)
            {
                return Err("Specified point does not exist");
            }
        } else if let Some(normal) = entity.normal() {
            if !self.entity_exists(normal) {
                return Err("Specified normal does not exist.");
            }
        } else if let Some(distance) = entity.distance() {
            if !self.entity_exists(distance) {
                return Err("Specified distance does not exist.");
            }
        }

        Ok(entity)
    }

    fn entity_exists(&self, h: Slvs_hEntity) -> bool {
        self.entities
            .list
            .binary_search_by_key(&h, |&Slvs_Entity { h, .. }| h)
            .is_ok()
    }

    fn h_to_slvs_entity(&self, h: Slvs_hEntity) -> Option<&Slvs_Entity> {
        self.entities
            .list
            .binary_search_by_key(&h, |&Slvs_Entity { h, .. }| h)
            .map_or(None, |ix| Some(&self.entities.list[ix]))
    }

    fn h_to_mut_slvs_entity(&mut self, h: Slvs_hEntity) -> Option<&mut Slvs_Entity> {
        self.entities
            .list
            .binary_search_by_key(&h, |&Slvs_Entity { h, .. }| h)
            .map_or(None, |ix| Some(&mut self.entities.list[ix]))
    }

    fn h_to_slvs_constraint(&self, h: Slvs_hConstraint) -> Option<&Slvs_Constraint> {
        self.constraints
            .list
            .binary_search_by_key(&h, |&Slvs_Constraint { h, .. }| h)
            .map_or(None, |ix| Some(&self.constraints.list[ix]))
    }

    fn h_to_some_constraint(&self, h: Slvs_hConstraint) -> Option<SomeConstraint> {
        self.h_to_slvs_constraint(h)
            .map(|Slvs_Constraint { h, type_, .. }| SomeConstraint::new(*type_ as _, *h))
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}
