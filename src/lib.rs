use std::any::Any;
use std::convert::identity;
use std::{iter::zip, marker::PhantomData};

use bindings::Slvs_hGroup;
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

trait AsHandle {
    fn as_handle(&self) -> u32;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group(u32);

impl AsHandle for Group {
    fn as_handle(&self) -> u32 {
        self.0
    }
}

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
// Creating Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn add_group(&mut self) -> Group {
        let new_group = Group(self.groups.get_next_h());

        self.groups.list.push(new_group);
        self.groups.list.last().cloned().unwrap()
    }

    pub fn add_entity<T: AsEntity>(
        &mut self,
        group: &Group,
        entity_data: T,
    ) -> Result<Entity<T>, &'static str> {
        self.validate_entity_data(&entity_data)?;

        let mut new_slvs_entity = Slvs_Entity::new(
            self.entities.get_next_h(),
            group.as_handle(),
            entity_data.type_(),
        );

        if let Some(workplane) = entity_data.workplane() {
            new_slvs_entity.set_workplane(workplane);
        }
        if let Some(points) = entity_data.points() {
            new_slvs_entity.set_point(points);
        }
        if let Some(normal) = entity_data.normal() {
            new_slvs_entity.set_normal(normal);
        }
        if let Some(distance) = entity_data.distance() {
            new_slvs_entity.set_distance(distance);
        }
        if let Some(param_vals) = entity_data.param_vals() {
            new_slvs_entity.set_param(
                param_vals
                    .into_iter()
                    .map(|val| self.add_param(group, val))
                    .collect(),
            );
        }

        self.entities.list.push(new_slvs_entity);
        Ok(Entity {
            handle: new_slvs_entity.h,
            phantom: PhantomData,
        })
    }

    pub fn add_constraint<T: AsConstraint>(
        &mut self,
        group: &Group,
        constraint: T,
    ) -> Result<Constraint<T>, &'static str> {
        let [point_a, point_b] = constraint.point();
        let [entity_a, entity_b, entity_c, entity_d] = constraint.entity();
        let [other, other_2] = constraint.other();

        let new_constraint = Slvs_Constraint {
            h: self.constraints.get_next_h(),
            group: group.as_handle(),
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
    fn add_param(&mut self, group: &Group, val: f64) -> Slvs_hParam {
        let new_param = Slvs_Param {
            h: self.params.get_next_h(),
            group: group.as_handle(),
            val,
        };

        self.params.list.push(new_param);
        self.params.list.last().unwrap().h
    }
}

////////////////////////////////////////////////////////////////////////////////
// Reading Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn get_entity_data<T>(&self, entity: &Entity<T>) -> Result<T, &'static str>
    where
        T: AsEntity + 'static,
    {
        self.slvs_entity(entity.as_handle()).map(|slvs_entity| {
            let some_entity_data: Box<dyn Any> = match slvs_entity.type_ as _ {
                SLVS_E_POINT_IN_3D => Box::new(PointIn3d::new(
                    self.slvs_param(slvs_entity.param[0]).unwrap().val,
                    self.slvs_param(slvs_entity.param[1]).unwrap().val,
                    self.slvs_param(slvs_entity.param[2]).unwrap().val,
                )),
                SLVS_E_POINT_IN_2D => todo!(),
                SLVS_E_NORMAL_IN_3D => todo!(),
                SLVS_E_NORMAL_IN_2D => todo!(),
                SLVS_E_DISTANCE => todo!(),
                SLVS_E_WORKPLANE => todo!(),
                SLVS_E_LINE_SEGMENT => Box::new(LineSegment::new_in_3d(
                    Entity::new(slvs_entity.point[0]),
                    Entity::new(slvs_entity.point[1]),
                )),
                SLVS_E_CUBIC => todo!(),
                SLVS_E_CIRCLE => todo!(),
                SLVS_E_ARC_OF_CIRCLE => todo!(),
                _ => panic!("Unknown entity type: {}", slvs_entity.type_),
            };

            *some_entity_data.downcast::<T>().unwrap()
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Updating Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    fn update_param(
        &mut self,
        h: Slvs_hParam,
        group_h: Option<Slvs_hGroup>,
        val: f64,
    ) -> Result<(), &'static str> {
        let mut param = self.mut_slvs_param(h)?;
        param.val = val;

        if let Some(group_h) = group_h {
            param.group = group_h;
        }

        Ok(())
    }

    pub fn update_entity<T, F>(
        &mut self,
        entity: &Entity<T>,
        f: F,
        group: Option<&Group>,
    ) -> Result<T, &'static str>
    where
        T: AsEntity + 'static,
        F: FnOnce(&mut T),
    {
        let mut entity_data = self.get_entity_data(entity)?;

        f(&mut entity_data);
        self.validate_entity_data(&entity_data)?;

        let param_h = {
            let slvs_entity = self.mut_slvs_entity(entity.as_handle()).unwrap();

            if let Some(group) = group {
                slvs_entity.set_group(group.as_handle())
            }
            if let Some(workplane) = entity_data.workplane() {
                slvs_entity.set_workplane(workplane)
            }
            if let Some(points) = entity_data.points() {
                slvs_entity.set_point(points);
            }
            if let Some(normal) = entity_data.normal() {
                slvs_entity.set_normal(normal);
            }
            if let Some(distance) = entity_data.distance() {
                slvs_entity.set_distance(distance);
            }

            slvs_entity.param
        };

        if let Some(param_vals) = entity_data.param_vals() {
            for (h, val) in zip(param_h, param_vals) {
                self.update_param(h, group.map(|g| (g.as_handle())), val)?;
            }
        }
        Ok(entity_data)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Deleting Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn delete_group(&mut self, group: Group) -> Result<Group, &'static str> {
        let ix = self.group_ix(group.as_handle())?;
        self.groups.list.remove(ix);
        Ok(group)
    }

    fn delete_param(&mut self, h: Slvs_hParam) -> Result<(), &'static str> {
        let ix = self.param_ix(h)?;
        self.params.list.remove(ix);

        Ok(())
    }

    pub fn delete_entity<T>(&mut self, entity: Entity<T>) -> Result<T, &'static str>
    where
        T: AsEntity + 'static,
    {
        let entity_data = self.get_entity_data(&entity)?;
        let ix = self.entity_ix(entity.as_handle())?;
        let deleted_entity = self.entities.list.remove(ix);

        for param_h in deleted_entity.param {
            self.delete_param(param_h)?
        }

        Ok(entity_data)
    }

    pub fn delete_constraint(&mut self) {
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Solving the system
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn set_dragged(&mut self, entity: &Entity<impl AsEntity>) {
        if let Ok(slvs_entity) = self.slvs_entity(entity.as_handle()) {
            self.dragged = slvs_entity.param;
        }
    }

    pub fn clear_dragged(&mut self) {
        self.dragged = [0; 4];
    }

    pub fn solve(&mut self, group: &Group) -> Result<SolveOkay, SolveFail> {
        let mut failed_handles: Vec<Slvs_hConstraint> = vec![0; self.constraints.list.len()];
        let mut slvs_system = Slvs_System::from(self, &mut failed_handles);

        unsafe {
            Slvs_Solve(&mut slvs_system, group.as_handle());
        };

        match FailReason::try_from(slvs_system.result) {
            Ok(fail_reason) => Err(SolveFail {
                dof: slvs_system.dof,
                reason: fail_reason,
                failed_constraints: failed_handles
                    .into_iter()
                    .map(|h| self.some_constraint(h).unwrap())
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
    fn group_ix(&self, h: Slvs_hGroup) -> Result<usize, &'static str> {
        self.groups
            .list
            .binary_search_by_key(&h, |g| g.as_handle())
            .map_err(|_| "Specified group not found.")
    }

    fn param_ix(&self, h: Slvs_hParam) -> Result<usize, &'static str> {
        self.params
            .list
            .binary_search_by_key(&h, |&Slvs_Param { h, .. }| h)
            .map_err(|_| "Specified parameter not found.")
    }

    fn slvs_param(&self, h: Slvs_hParam) -> Result<&Slvs_Param, &'static str> {
        let ix = self.param_ix(h)?;
        Ok(&self.params.list[ix])
    }

    fn mut_slvs_param(&mut self, h: Slvs_hParam) -> Result<&mut Slvs_Param, &'static str> {
        let ix = self.param_ix(h)?;
        Ok(&mut self.params.list[ix])
    }

    fn entity_ix(&self, h: Slvs_hEntity) -> Result<usize, &'static str> {
        self.entities
            .list
            .binary_search_by_key(&h, |&Slvs_Entity { h, .. }| h)
            .map_err(|_| "Specified entity not found.")
    }

    fn slvs_entity(&self, h: Slvs_hEntity) -> Result<&Slvs_Entity, &'static str> {
        let ix = self.entity_ix(h)?;
        Ok(&self.entities.list[ix])
    }

    fn mut_slvs_entity(&mut self, h: Slvs_hEntity) -> Result<&mut Slvs_Entity, &'static str> {
        let ix = self.entity_ix(h)?;
        Ok(&mut self.entities.list[ix])
    }

    // Checks that all elements referenced within entity_data exist
    // TODO: If there is a workplane, check that all entities references lie on the same workplane
    fn validate_entity_data<T: AsEntity>(&self, entity_data: &T) -> Result<(), &'static str> {
        if let Some(workplane) = entity_data.workplane() {
            if self.entity_ix(workplane).is_err() {
                return Err("Specified workplane not found.");
            }
        } else if let Some(points) = entity_data.points() {
            if points
                .into_iter()
                .map(|point| self.entity_ix(point).is_err())
                .any(identity)
            {
                return Err("Specified point not found");
            }
        } else if let Some(normal) = entity_data.normal() {
            if self.entity_ix(normal).is_err() {
                return Err("Specified normal not found.");
            }
        } else if let Some(distance) = entity_data.distance() {
            if self.entity_ix(distance).is_err() {
                return Err("Specified distance not found.");
            }
        }

        Ok(())
    }

    fn constraint_ix(&self, h: Slvs_hConstraint) -> Result<usize, &'static str> {
        self.constraints
            .list
            .binary_search_by_key(&h, |&Slvs_Constraint { h, .. }| h)
            .map_err(|_| "Specified constraint not found.")
    }

    fn slvs_constraint(&self, h: Slvs_hConstraint) -> Result<&Slvs_Constraint, &'static str> {
        let ix = self.constraint_ix(h)?;
        Ok(&self.constraints.list[ix])
    }

    fn mut_slvs_constraint(
        &mut self,
        h: Slvs_hConstraint,
    ) -> Result<&mut Slvs_Constraint, &'static str> {
        let ix = self.constraint_ix(h)?;
        Ok(&mut self.constraints.list[ix])
    }

    fn some_constraint(&self, h: Slvs_hConstraint) -> Result<SomeConstraint, &'static str> {
        self.slvs_constraint(h)
            .map(|Slvs_Constraint { h, type_, .. }| SomeConstraint::new(*type_ as _, *h))
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}
