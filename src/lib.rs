use std::any::Any;
use std::{iter::zip, marker::PhantomData};

use binding::{Slvs_Constraint, Slvs_hConstraint};
use binding::{Slvs_Entity, Slvs_hEntity};
use binding::{Slvs_Param, Slvs_hParam};
use binding::{
    Slvs_Solve, Slvs_System, SLVS_RESULT_DIDNT_CONVERGE, SLVS_RESULT_INCONSISTENT,
    SLVS_RESULT_TOO_MANY_UNKNOWNS,
};
use constraint::{AsConstraint, Constraint, SomeConstraint};
use entity::{AsEntity, Entity, EntityData, LineSegment, PointIn3d, SomeEntity};

mod binding;
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

    pub fn add_entity<T: AsEntity>(
        &mut self,
        group: Group,
        entity_data: T,
    ) -> Result<Entity<T>, &'static str> {
        let new_entity = Slvs_Entity {
            h: self.entities.get_next_h(),
            group: group.into(),
            type_: entity_data.type_() as _,
            wrkpl: entity_data.workplane().unwrap_or(0), // TODO: check that entity exists and is the correct type
            point: entity_data.point().map(|p| p.unwrap_or(0)), // TODO: ditto
            normal: entity_data.normal().unwrap_or(0),   // TODO: ditto
            distance: entity_data.distance().unwrap_or(0), // TODO: ditto
            param: entity_data
                .param_vals()
                .map(|opt_val| opt_val.map_or(0, |v| self.add_param(group, v))),
        };

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
        Entity<T>: Into<SomeEntity> + Into<Slvs_hEntity>,
    {
        self.h_to_slvs_entity(entity.into()).map(|slvs_entity| {
            let some_entity: Box<dyn Any> = match entity.into() {
                SomeEntity::PointIn3d(_) => Box::new(PointIn3d {
                    x: self.h_to_slvs_param(slvs_entity.param[0]).unwrap().val,
                    y: self.h_to_slvs_param(slvs_entity.param[1]).unwrap().val,
                    z: self.h_to_slvs_param(slvs_entity.param[2]).unwrap().val,
                }),
                SomeEntity::LineSegment(_) => Box::new(LineSegment {
                    point_a: Entity::new(slvs_entity.point[0]),
                    point_b: Entity::new(slvs_entity.point[1]),
                }),
            };

            *some_entity.downcast_ref::<T>().unwrap()
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Updating Elements
///////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn update_entity<T, F>(&mut self, entity: Entity<T>, f: F) -> Result<T, &'static str>
    where
        T: AsEntity + TryFrom<EntityData, Error = &'static str> + Copy + 'static,
        Entity<T>: Into<SomeEntity> + Into<Slvs_hEntity>,
        F: FnOnce(&mut T),
    {
        if let Some(mut entity_data) = self.get_entity_data(entity) {
            f(&mut entity_data);

            // scoped to allow another mut self for the params.
            let param_h = {
                // safe to unwrap() we've already confirmed `entity` exists in the `if let` above
                let mut slvs_entity = self.h_to_mut_slvs_entity(entity.into()).unwrap();

                slvs_entity.wrkpl = entity_data.workplane().unwrap_or(0);
                slvs_entity.point = entity_data.point().map(|p| p.unwrap_or(0));
                slvs_entity.normal = entity_data.normal().unwrap_or(0);
                slvs_entity.distance = entity_data.distance().unwrap_or(0);

                slvs_entity.param
            };

            let mut h_val_iter = zip(param_h, entity_data.param_vals());
            while let Some((h, Some(val))) = h_val_iter.next() {
                self.update_param(h, val)?;
            }

            Ok(entity_data)
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

    fn h_to_some_entity(&self, h: Slvs_hEntity) -> Option<SomeEntity> {
        self.h_to_slvs_entity(h)
            .map(|Slvs_Entity { h, type_, .. }| SomeEntity::new(*type_ as _, *h))
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
