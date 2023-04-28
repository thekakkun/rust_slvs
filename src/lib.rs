use std::iter::zip;
use std::marker::PhantomData;

mod bindings;
use bindings::Slvs_hGroup;
pub use bindings::{make_quaternion, quaternion_n, quaternion_u, quaternion_v};
use bindings::{
    Slvs_Constraint, Slvs_Entity, Slvs_Param, Slvs_hConstraint, Slvs_hEntity, Slvs_hParam,
    SLVS_E_NORMAL_IN_3D,
};

pub mod solver;

mod element;
use element::{AsHandle, SlvsElements};

pub mod group;
use group::Group;

pub mod target;
use target::AsTarget;

pub mod entity;
use entity::{AsEntity, AsEntityData, Entity, FromSlvsEntity};

pub mod constraint;
use constraint::{AsConstraint, AsConstraintData, Constraint};

#[derive(Debug)]
pub struct System {
    pub groups: Vec<Group>,
    pub entities: Vec<Box<dyn AsEntity>>,
    pub constraints: Vec<Box<dyn AsConstraint>>,
    pub calculate_faileds: bool,
    slvs: SlvsElements,
    dragged: [Slvs_hParam; 4],
}

impl System {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            entities: Vec::new(),
            constraints: Vec::new(),
            calculate_faileds: true,
            slvs: SlvsElements::new(),
            dragged: [0; 4],
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Creating Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn add_group(&mut self) -> Group {
        let new_group = Group(self.slvs.groups.get_next_h());
        self.groups.push(new_group);

        self.slvs.groups.list.push(new_group);
        self.slvs.groups.list.last().cloned().unwrap()
    }

    pub fn sketch<E: AsEntityData + 'static>(
        &mut self,
        group: &Group,
        entity_data: E,
    ) -> Result<Entity<E>, &'static str> {
        self.validate_entity_data(&entity_data)?;

        let mut new_slvs_entity = Slvs_Entity::new(
            self.slvs.entities.get_next_h(),
            group.handle(),
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

        self.slvs.entities.list.push(new_slvs_entity);

        let entity: Entity<E> = Entity {
            handle: new_slvs_entity.h,
            phantom: PhantomData,
        };
        self.entities.push(Box::new(entity));

        Ok(entity)
    }

    pub fn constrain<C: AsConstraintData + 'static>(
        &mut self,
        group: &Group,
        constraint_data: C,
    ) -> Result<Constraint<C>, &'static str> {
        self.validate_constraint_data(&constraint_data)?;

        let mut new_slvs_constraint = Slvs_Constraint::new(
            self.slvs.constraints.get_next_h(),
            group.handle(),
            constraint_data.type_(),
        );

        if let Some(workplane) = constraint_data.workplane() {
            new_slvs_constraint.set_workplane(workplane)
        }
        if let Some(val) = constraint_data.val() {
            new_slvs_constraint.set_val(val);
        }
        if let Some(points) = constraint_data.points() {
            new_slvs_constraint.set_points(points);
        }
        if let Some(entities) = constraint_data.entities() {
            new_slvs_constraint.set_entities(entities)
        }
        new_slvs_constraint.set_others(constraint_data.others());

        self.slvs.constraints.list.push(new_slvs_constraint);

        let constraint = Constraint::new(new_slvs_constraint.h);
        self.constraints.push(Box::new(constraint));

        Ok(constraint)
    }

    // Private as user has no reason to create bare param without linking to an entity.
    fn add_param(&mut self, group: &Group, val: f64) -> Slvs_hParam {
        let new_param = Slvs_Param {
            h: self.slvs.params.get_next_h(),
            group: group.handle(),
            val,
        };

        self.slvs.params.list.push(new_param);
        self.slvs.params.list.last().unwrap().h
    }
}

////////////////////////////////////////////////////////////////////////////////
// Reading Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn entity_data<E, T>(&self, entity: &Entity<E>) -> Result<E, &'static str>
    where
        E: FromSlvsEntity<T>,
        T: AsTarget,
    {
        let slvs_entity = self.slvs_entity(entity.handle())?;
        let mut entity_data = E::from(*slvs_entity);

        let param_vals: Vec<_> = slvs_entity
            .param
            .iter()
            .filter_map(|&param_h| {
                if param_h == 0 {
                    None
                } else if let Ok(slvs_param) = self.slvs_param(param_h) {
                    Some(slvs_param.val)
                } else {
                    None
                }
            })
            .collect();

        if !param_vals.is_empty() {
            entity_data.set_vals(param_vals);
        }

        Ok(entity_data)
    }

    pub fn constraint_data<C: AsConstraintData + From<Slvs_Constraint>>(
        &self,
        constraint: &Constraint<C>,
    ) -> Result<C, &'static str> {
        let slvs_constraint = self.slvs_constraint(constraint.handle())?;
        Ok((*slvs_constraint).into())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Updating Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    fn update_param(&mut self, h: Slvs_hParam, val: f64) -> Result<(), &'static str> {
        let mut param = self.mut_slvs_param(h)?;
        param.val = val;

        Ok(())
    }

    pub fn update_entity<E, T, F>(&mut self, entity: &Entity<E>, f: F) -> Result<E, &'static str>
    where
        E: FromSlvsEntity<T>,
        T: AsTarget,
        F: FnOnce(&mut E),
    {
        let mut entity_data = self.entity_data(entity)?;

        f(&mut entity_data);
        self.validate_entity_data(&entity_data)?;

        let param_h = {
            let slvs_entity = self.mut_slvs_entity(entity.handle()).unwrap();

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
                self.update_param(h, val)?;
            }
        }
        Ok(entity_data)
    }

    pub fn update_constraint<C, F>(
        &mut self,
        constraint: &Constraint<C>,
        f: F,
    ) -> Result<C, &'static str>
    where
        C: AsConstraintData + From<Slvs_Constraint>,
        F: FnOnce(&mut C),
    {
        let mut constraint_data = self.constraint_data(constraint)?;

        f(&mut constraint_data);
        self.validate_constraint_data(&constraint_data)?;

        let slvs_constraint = self.mut_slvs_constraint(constraint.handle()).unwrap();

        if let Some(val) = constraint_data.val() {
            slvs_constraint.set_val(val);
        }
        if let Some(points) = constraint_data.points() {
            slvs_constraint.set_points(points);
        }
        if let Some(entities) = constraint_data.entities() {
            slvs_constraint.set_entities(entities);
        }
        slvs_constraint.set_others(constraint_data.others());

        Ok(constraint_data)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Deleting Elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn delete_group(&mut self, group: Group) -> Result<Group, &'static str> {
        let ix = self.group_ix(group.handle())?;
        self.slvs.groups.list.remove(ix);
        self.groups.remove(ix);
        Ok(group)
    }

    fn delete_param(&mut self, h: Slvs_hParam) -> Result<(), &'static str> {
        let ix = self.param_ix(h)?;
        self.slvs.params.list.remove(ix);

        Ok(())
    }

    pub fn delete_entity<E, T>(&mut self, entity: Entity<E>) -> Result<E, &'static str>
    where
        E: FromSlvsEntity<T>,
        T: AsTarget,
    {
        let entity_data = self.entity_data(&entity)?;
        let ix = self.entity_ix(entity.handle())?;
        let deleted_entity = self.slvs.entities.list.remove(ix);
        self.entities.remove(ix);

        for param_h in deleted_entity.param {
            self.delete_param(param_h)?
        }

        Ok(entity_data)
    }

    pub fn delete_constraint<C: AsConstraintData + From<Slvs_Constraint>>(
        &mut self,
        constraint: Constraint<C>,
    ) -> Result<C, &'static str> {
        let constraint_data = self.constraint_data(&constraint)?;

        let ix = self.constraint_ix(constraint.handle())?;
        self.slvs.constraints.list.remove(ix);
        self.constraints.remove(ix);

        Ok(constraint_data)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Internal methods for Slvs_Handles -> other stuff
////////////////////////////////////////////////////////////////////////////////

impl System {
    fn group_ix(&self, h: Slvs_hGroup) -> Result<usize, &'static str> {
        self.slvs
            .groups
            .list
            .binary_search_by_key(&h, |g| g.handle())
            .map_err(|_| "Specified group not found.")
    }

    fn param_ix(&self, h: Slvs_hParam) -> Result<usize, &'static str> {
        self.slvs
            .params
            .list
            .binary_search_by_key(&h, |&Slvs_Param { h, .. }| h)
            .map_err(|_| "Specified parameter not found.")
    }

    fn slvs_param(&self, h: Slvs_hParam) -> Result<&Slvs_Param, &'static str> {
        let ix = self.param_ix(h)?;
        Ok(&self.slvs.params.list[ix])
    }

    fn mut_slvs_param(&mut self, h: Slvs_hParam) -> Result<&mut Slvs_Param, &'static str> {
        let ix = self.param_ix(h)?;
        Ok(&mut self.slvs.params.list[ix])
    }

    fn entity_ix(&self, h: Slvs_hEntity) -> Result<usize, &'static str> {
        self.slvs
            .entities
            .list
            .binary_search_by_key(&h, |&Slvs_Entity { h, .. }| h)
            .map_err(|_| "Specified entity not found.")
    }

    fn slvs_entity(&self, h: Slvs_hEntity) -> Result<&Slvs_Entity, &'static str> {
        let ix = self.entity_ix(h)?;
        Ok(&self.slvs.entities.list[ix])
    }

    fn entity_on_workplane(
        &self,
        h: Slvs_hEntity,
        workplane: Slvs_hEntity,
    ) -> Result<(), &'static str> {
        let entity = self.slvs_entity(h)?;

        match entity.type_ as _ {
            SLVS_E_NORMAL_IN_3D => match entity.h == self.slvs_entity(workplane)?.normal {
                true => Ok(()),
                false => Err("Normal in 3d does not match workplane's normal."),
            },
            _ => match entity.wrkpl == workplane {
                true => Ok(()),
                false => Err("Entity not on expected workplane."),
            },
        }
    }

    fn mut_slvs_entity(&mut self, h: Slvs_hEntity) -> Result<&mut Slvs_Entity, &'static str> {
        let ix = self.entity_ix(h)?;
        Ok(&mut self.slvs.entities.list[ix])
    }

    // Checks that all elements referenced within entity_data exist and are on the expected workplane
    fn validate_entity_data(&self, entity_data: &impl AsEntityData) -> Result<(), &'static str> {
        if let Some(points) = entity_data.points() {
            let all_points_valid: Result<Vec<_>, _> = points
                .into_iter()
                .map(|point| {
                    if let Some(workplane_h) = entity_data.workplane() {
                        self.entity_on_workplane(point, workplane_h)
                            .map_err(|_| "Point not on expected workplane.")
                    } else {
                        self.slvs_entity(point).map(|_| ())
                    }
                })
                .collect();
            all_points_valid?;
        }

        if let Some(normal) = entity_data.normal() {
            let normal_valid = if let Some(workplane_h) = entity_data.workplane() {
                self.entity_on_workplane(normal, workplane_h)
                    .map_err(|_| "Normal not on expected workplane.")
            } else {
                self.slvs_entity(normal).map(|_| ())
            };
            normal_valid?;
        }

        if let Some(distance) = entity_data.distance() {
            let distance_valid = if let Some(workplane_h) = entity_data.workplane() {
                self.entity_on_workplane(distance, workplane_h)
                    .map_err(|_| "Distance not on expected workplane.")
            } else {
                self.slvs_entity(distance).map(|_| ())
            };
            distance_valid?;
        }

        Ok(())
    }

    fn constraint_ix(&self, h: Slvs_hConstraint) -> Result<usize, &'static str> {
        self.slvs
            .constraints
            .list
            .binary_search_by_key(&h, |&Slvs_Constraint { h, .. }| h)
            .map_err(|_| "Specified constraint not found.")
    }

    fn slvs_constraint(&self, h: Slvs_hConstraint) -> Result<&Slvs_Constraint, &'static str> {
        let ix = self.constraint_ix(h)?;
        Ok(&self.slvs.constraints.list[ix])
    }

    fn mut_slvs_constraint(
        &mut self,
        h: Slvs_hConstraint,
    ) -> Result<&mut Slvs_Constraint, &'static str> {
        let ix = self.constraint_ix(h)?;
        Ok(&mut self.slvs.constraints.list[ix])
    }

    fn validate_constraint_data(
        &self,
        constraint_data: &impl AsConstraintData,
    ) -> Result<(), &'static str> {
        if let Some(points) = constraint_data.points() {
            let all_points_valid: Result<Vec<_>, _> = points
                .into_iter()
                .map(|point| self.slvs_entity(point).map(|_| ()))
                .collect();
            all_points_valid?;
        }

        if let Some(entities) = constraint_data.entities() {
            let all_entities_valid: Result<Vec<_>, _> = entities
                .into_iter()
                .map(|entity| self.slvs_entity(entity).map(|_| ()))
                .collect();
            all_entities_valid?;
        }

        Ok(())
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}
