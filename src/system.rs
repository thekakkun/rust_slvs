/*!
Object used to store and interact with all the elements and constraints in the sketch.
*/

use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

use crate::{
    bindings::{
        Slvs_Constraint, Slvs_Entity, Slvs_Param, Slvs_Solve, Slvs_System, Slvs_hConstraint,
        Slvs_hEntity, Slvs_hGroup, Slvs_hParam, SLVS_C_CURVE_CURVE_TANGENT, SLVS_C_DIAMETER,
        SLVS_C_EQUAL_RADIUS, SLVS_C_PROJ_PT_DISTANCE, SLVS_C_PT_ON_CIRCLE, SLVS_E_ARC_OF_CIRCLE,
        SLVS_E_CIRCLE, SLVS_E_CUBIC, SLVS_E_DISTANCE, SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D,
        SLVS_E_NORMAL_IN_3D, SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D, SLVS_E_WORKPLANE,
        SLVS_FREE_IN_3D, SLVS_RESULT_DIDNT_CONVERGE, SLVS_RESULT_INCONSISTENT, SLVS_RESULT_OKAY,
        SLVS_RESULT_TOO_MANY_UNKNOWNS,
    },
    constraint::{
        AsConstraintData, AsConstraintHandle, ConstraintHandle, CurveCurveTangent, Diameter,
        EqualRadius, ProjPtDistance, PtOnCircle,
    },
    element::AsHandle,
    entity::{
        ArcOfCircle, AsEntityData, AsEntityHandle, Circle, Cubic, EntityHandle, LineSegment, Normal,
    },
    group::Group,
};

/// Wrapper around the SolveSpace C structs.
///
/// This object is used to store the list of structs expected by the C library.
/// See the [header file](https://github.com/solvespace/solvespace/blob/master/include/slvs.h)
/// for what these structs should look like.
#[derive(Debug, Serialize)]
pub struct Elements<T> {
    pub list: Vec<T>,
    next_h: u32,
}

impl<T> Elements<T> {
    fn new() -> Self {
        Self {
            list: Vec::new(),
            next_h: 1,
        }
    }

    fn next_h(&mut self) -> u32 {
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

/// Object stores all data regarding parameters, entities, and constraints.
#[derive(Debug, Serialize)]
pub struct System {
    /// Groups in the system. Not used within the SolveSpace library, but useful
    /// to see what groups exist in the system.
    pub groups: Elements<Group>,
    /// Params as used by the SolveSpace library. Unlike the original C library, these
    /// are automatically created/updated as necessary, and methods are not surfaced for
    /// the user to interact with them.
    pub params: Elements<Slvs_Param>,
    pub entities: Elements<Slvs_Entity>,
    pub constraints: Elements<Slvs_Constraint>,
    /// Sets whether the solver tries to figure out what constraints failed, which can
    /// be a relatively slow process.
    pub calculate_faileds: bool,
    pub(crate) dragged: [Slvs_hParam; 4],
}

impl System {
    pub fn new() -> Self {
        Self {
            groups: Elements::new(),
            params: Elements::new(),
            entities: Elements::new(),
            constraints: Elements::new(),
            calculate_faileds: true,
            dragged: [0; 4],
        }
    }
}

impl System {
    /// Add a new `Group` to the `System`
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{System, group::Group};
    ///
    /// let mut sys = System::new();
    /// let group: Group = sys.add_group();
    /// ```
    pub fn add_group(&mut self) -> Group {
        let new_group = Group(self.groups.next_h());

        self.groups.list.push(new_group);
        self.groups.list.last().cloned().unwrap()
    }

    /// Add a new [entity][crate::entity] to the system.
    ///
    /// # Arguments
    ///
    /// * `entity_data` - A struct defining some sort of `EntityData`
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{System, entity::Point};
    ///
    /// let mut sys = System::new();
    /// let group = sys.add_group();
    /// sys.sketch(Point::new_in_3d(group, [10.0, 20.0, 30.0]));
    /// ```
    ///
    /// # Errors
    ///
    /// If any of the entities referenced in `entity_data` are not found in the system,
    /// an error will be returned.
    pub fn sketch<E: AsEntityData>(
        &mut self,
        entity_data: E,
    ) -> Result<EntityHandle<E>, &'static str> {
        let workplane_h = self.sketch_target(&entity_data)?;

        // ArcOfCircle requires a Normal, which is identical to its workplane's normal
        let normal_h = if SLVS_E_ARC_OF_CIRCLE == entity_data.slvs_type() as _ {
            let slvs_workplane = self.slvs_entity(entity_data.workplane().unwrap())?;
            slvs_workplane.normal
        } else {
            entity_data.normal().unwrap_or(0)
        };

        let param_h = entity_data.param_vals().map(|val| match val {
            Some(val) => self.add_param(entity_data.group(), val),
            None => 0,
        });

        let slvs_entity = Slvs_Entity {
            h: self.entities.next_h(),
            group: entity_data.group(),
            type_: entity_data.slvs_type(),
            wrkpl: workplane_h.unwrap_or(SLVS_FREE_IN_3D),
            point: entity_data.points().unwrap_or([0; 4]),
            normal: normal_h,
            distance: entity_data.distance().unwrap_or(0),
            param: param_h,
        };

        self.entities.list.push(slvs_entity);

        // For an arc, needs a solve step to ensure that
        // distance(center, start) = distance(center, end)
        // if SLVS_E_ARC_OF_CIRCLE == entity_data.slvs_type() as _ {
        //     self.solve(&Group(entity_data.group()));
        // }
        let entity_handle = EntityHandle::new(slvs_entity.h);

        Ok(entity_handle)
    }

    /// [Constrain][crate::constraint] entities within the system.
    ///
    /// Note that constraints are applied to the system, only once [`System::solve`] has been called.
    ///
    /// # Arguments
    ///
    /// * `constraint_data` - A struct defining some sort of `ConstraintData`
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{System, constraint::PointsCoincident, entity::Point};
    ///
    /// let mut sys = System::new();
    /// let group = sys.add_group();
    /// let p1 = sys
    ///     .sketch(Point::new_in_3d(group, [0.0, 0.0, 0.0]))
    ///     .expect("point created at (0, 0, 0)");
    /// let p2 = sys
    ///     .sketch(Point::new_in_3d(group, [10.0, 20.0, 30.0]))
    ///     .expect("point created at (10, 20, 30");
    /// sys.constrain(PointsCoincident::new(group, p1, p2, None))
    ///     .expect("Constraint added so that p1 and p2 are coincident");
    /// ```
    ///
    /// # Errors
    ///
    /// If any of the entities referenced in `constraint_data` are not found in the system,
    /// an error will be returned.
    pub fn constrain<C: AsConstraintData>(
        &mut self,
        constraint_data: C,
    ) -> Result<ConstraintHandle<C>, &'static str> {
        let workplane_h = self.constraint_target(&constraint_data)?;

        let [pt_a, pt_b] = constraint_data.points().unwrap_or([0; 2]);
        let [entity_a, entity_b, entity_c, entity_d] = constraint_data.entities().unwrap_or([0; 4]);
        let [other, other2] = constraint_data.others();

        let slvs_constraint = Slvs_Constraint {
            h: self.constraints.next_h(),
            group: constraint_data.group(),
            type_: constraint_data.slvs_type(),
            wrkpl: workplane_h.unwrap_or(SLVS_FREE_IN_3D),
            valA: constraint_data.val().unwrap_or(0.0),
            ptA: pt_a,
            ptB: pt_b,
            entityA: entity_a,
            entityB: entity_b,
            entityC: entity_c,
            entityD: entity_d,
            other: other as _,
            other2: other2 as _,
        };

        self.constraints.list.push(slvs_constraint);

        let constraint_handle = ConstraintHandle::new(slvs_constraint.h);

        Ok(constraint_handle)
    }

    /// Get a list of groups within the system.
    pub fn groups(&self) -> Vec<Group> {
        self.groups.list.clone()
    }

    /// Get a list of the entity handles within the system
    ///
    /// # Arguments
    ///
    /// * `group` - If provided, only returns handles for entities belonging to group.
    /// * `entity_handle` - If provided, only returns handles for entities that reference this entity.
    /// Note that the returned vector will not include the handle for the one specified here.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{
    ///   entity::{LineSegment, Point},
    ///   System,
    /// };
    ///
    /// let mut sys = System::new();
    /// let g1 = sys.add_group();
    /// let p1 = sys
    ///     .sketch(Point::new_in_3d(g1, [0.0, 0.0, 0.0]))
    ///     .expect("p1 belongs in g1");
    /// let g2 = sys.add_group();
    /// let p2 = sys
    ///     .sketch(Point::new_in_3d(g2, [10.0, 10.0, 10.0]))
    ///     .expect("p2 belongs in g1");
    ///
    /// let line = sys
    ///     .sketch(LineSegment::new(g1, p1, p2))
    ///     .expect("line 1 belongs in g1, references p1 and p2");
    ///
    /// let entity_handles = sys.entity_handles(None, None);
    /// assert!(entity_handles.iter().any(|e| Ok(p1) == e.try_into()));
    /// assert!(entity_handles.iter().any(|e| Ok(p2) == e.try_into()));
    /// assert!(entity_handles.iter().any(|e| Ok(line) == e.try_into()));
    ///
    /// // p2 is the only entity in g2
    /// let g2_entity_handles = sys.entity_handles(Some(&g2), None);
    /// assert!(!g2_entity_handles.iter().any(|e| Ok(p1) == e.try_into()));
    /// assert!(g2_entity_handles.iter().any(|e| Ok(p2) == e.try_into()));
    /// assert!(!g2_entity_handles.iter().any(|e| Ok(line) == e.try_into()));
    ///
    /// // line is the only entity that references p1
    /// let p1_entity_handles = sys.entity_handles(None, Some(&p1));
    /// assert!(!p1_entity_handles.iter().any(|e| Ok(p1) == e.try_into()));
    /// assert!(!p1_entity_handles.iter().any(|e| Ok(p2) == e.try_into()));
    /// assert!(p1_entity_handles.iter().any(|e| Ok(line) == e.try_into()));
    /// ```
    pub fn entity_handles(
        &self,
        group: Option<&Group>,
        entity_handle: Option<&dyn AsEntityHandle>,
    ) -> Vec<Box<dyn AsEntityHandle>> {
        self.entities
            .list
            .iter()
            .filter(|&slvs_entity| group.map_or(true, |group| slvs_entity.group == group.handle()))
            .filter(|&slvs_entity| {
                entity_handle.map_or(true, |entity_handle| {
                    [slvs_entity.wrkpl, slvs_entity.normal, slvs_entity.distance]
                        .contains(&entity_handle.handle())
                        | slvs_entity.point.contains(&entity_handle.handle())
                })
            })
            .map(|&slvs_entity| slvs_entity.into())
            .collect()
    }

    /// Get the data for the entity referenced by handle.
    ///
    /// # Arguments
    ///
    /// * `entity_handle` - Handle for the entity you want to get data on.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{entity::Point, System};
    ///
    /// let mut sys = System::new();
    /// let g = sys.add_group();
    ///
    /// let p = sys
    ///     .sketch(Point::new_in_3d(g, [10.0, 20.0, 30.0]))
    ///     .expect("point in 3d created at (10, 20, 30)");
    ///
    /// let p_data = sys.entity_data(&p).expect("point found");
    /// if let Point::In3d {
    ///     coords: [x, y, z], ..
    /// } = p_data
    /// {
    ///     assert_eq!(x, 10.0);
    ///     assert_eq!(y, 20.0);
    ///     assert_eq!(z, 30.0);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if entity with that handle does not exist, or if entity data
    /// within the system is not for entity of type `E`.
    pub fn entity_data<E: AsEntityData>(
        &self,
        entity_handle: &EntityHandle<E>,
    ) -> Result<E, &'static str> {
        E::from_system(self, entity_handle)
    }

    /// Get a list of the constraint handles within the system
    ///
    /// # Arguments
    ///
    /// * `group` - If provided, only returns handles for constraints belonging to group.
    /// * `entity_handle` - If provided, only returns handles for constraints acting upon entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{
    ///     constraint::{PointsCoincident, PtPtDistance},
    ///     entity::Point,
    ///     System,
    /// };
    ///
    /// let mut sys = System::new();
    /// let g = sys.add_group();
    /// let p1 = sys
    ///     .sketch(Point::new_in_3d(g, [0.0, 0.0, 0.0]))
    ///     .expect("p1 created");
    /// let p2 = sys
    ///     .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
    ///     .expect("p2 created");
    /// let p3 = sys
    ///     .sketch(Point::new_in_3d(g, [20.0, 20.0, 20.0]))
    ///     .expect("p2 created");
    ///
    /// let p_coincident = sys
    ///     .constrain(PointsCoincident::new(g, p1, p2, None))
    ///     .expect("p1 and p2 are coincident");
    /// let p_distance = sys
    ///     .constrain(PtPtDistance::new(g, p2, p3, 10.0, None))
    ///     .expect("p2 and p3 are 10 units apart");
    ///
    /// let constraint_handles = sys.constraint_handles(None, None);
    /// assert!(constraint_handles
    ///     .iter()
    ///     .any(|c| Ok(p_coincident) == c.try_into()));
    /// assert!(constraint_handles
    ///     .iter()
    ///     .any(|c| Ok(p_distance) == c.try_into()));
    ///
    /// let p1_constraint_handles = sys.constraint_handles(None, Some(&p1));
    /// assert!(p1_constraint_handles
    ///     .iter()
    ///     .any(|c| Ok(p_coincident) == c.try_into()));
    /// assert!(!p1_constraint_handles
    ///     .iter()
    ///     .any(|c| Ok(p_distance) == c.try_into()));
    /// ```
    pub fn constraint_handles(
        &self,
        group: Option<&Group>,
        entity_handle: Option<&dyn AsEntityHandle>,
    ) -> Vec<Box<dyn AsConstraintHandle>> {
        self.constraints
            .list
            .iter()
            .filter(|&slvs_constraint| {
                group.map_or(true, |group| slvs_constraint.group == group.handle())
            })
            .filter(|&slvs_constraint| {
                entity_handle.map_or(true, |entity_handle| {
                    [
                        slvs_constraint.ptA,
                        slvs_constraint.ptB,
                        slvs_constraint.entityA,
                        slvs_constraint.entityB,
                        slvs_constraint.entityC,
                        slvs_constraint.entityD,
                    ]
                    .contains(&entity_handle.handle())
                })
            })
            .map(|&slvs_constraint| self.boxed_constraint_handle(slvs_constraint))
            .collect()
    }

    /// Get the data for the constraint referenced by handle.
    ///
    /// # Arguments
    /// * `constraint_handle` - Handle for the constraint you want data on.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{constraint::PointsCoincident, entity::Point, System};
    ///
    /// let mut sys = System::new();

    /// let g = sys.add_group();
    /// let p1 = sys
    ///     .sketch(Point::new_in_3d(g, [0.0, 0.0, 0.0]))
    ///     .expect("p1 created");
    /// let p2 = sys
    ///     .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
    ///     .expect("p2 created");

    /// let p_coincident = sys
    ///     .constrain(PointsCoincident::new(g, p1, p2, None))
    ///     .expect("p1 and p2 are coincident");

    /// println!(
    ///     "{:#?}",
    ///     sys.constraint_data(&p_coincident)
    ///         .expect("coincident data found")
    /// );
    /// ```
    pub fn constraint_data<C: AsConstraintData>(
        &self,
        constraint_handle: &ConstraintHandle<C>,
    ) -> Result<C, &'static str> {
        C::from_system(self, constraint_handle)
    }

    /// Update the entity data within the system, and return the updated entity data.
    ///
    /// # Arguments
    ///
    /// * `entity_handle` - Handle for the entity you want to update.
    /// * `f` - Function that takes a reference to the entity data and mutates it.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{entity::Point, System};
    ///
    /// let mut sys = System::new();
    /// let g = sys.add_group();
    /// let p = sys
    ///     .sketch(Point::new_in_3d(g, [0.0, 0.0, 0.0]))
    ///     .expect("point in 3d created");
    ///
    /// let updated_p_x = 10.0;
    /// let updated_p_y = 20.0;
    /// let updated_p_z = 30.0;
    ///
    /// let updated_p_data = sys
    ///     .update_entity(&p, |entity| {
    ///         if let Point::In3d { ref mut coords, .. } = entity {
    ///             *coords = [updated_p_x, updated_p_y, updated_p_z]
    ///         }
    ///     })
    ///     .expect("should get updated point data");
    ///
    /// if let Point::In3d {
    ///     coords: [x, y, z], ..
    /// } = updated_p_data
    /// {
    ///     assert_eq!(x, updated_p_x);
    ///     assert_eq!(y, updated_p_y);
    ///     assert_eq!(z, updated_p_z);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if specified entity does not exist, or if entity data references
    /// any entity that does not exist.
    pub fn update_entity<E, F>(
        &mut self,
        entity_handle: &EntityHandle<E>,
        f: F,
    ) -> Result<E, &'static str>
    where
        E: AsEntityData,
        F: FnOnce(&mut E),
    {
        let mut entity_data = self.entity_data(entity_handle)?;
        f(&mut entity_data);

        let workplane_h = self.sketch_target(&entity_data)?;

        // ArcOfCircle requires a Normal, which is identical to its workplane's normal
        let normal_h = if SLVS_E_ARC_OF_CIRCLE == entity_data.slvs_type() as _ {
            let slvs_workplane = self.slvs_entity(entity_data.workplane().unwrap())?;
            slvs_workplane.normal
        } else {
            entity_data.normal().unwrap_or(0)
        };

        let param_h = {
            let slvs_entity = self.mut_slvs_entity(entity_handle.handle())?;

            slvs_entity.group = entity_data.group();
            slvs_entity.wrkpl = workplane_h.unwrap_or(SLVS_FREE_IN_3D);
            slvs_entity.point = entity_data.points().unwrap_or([0; 4]);
            slvs_entity.normal = normal_h;
            slvs_entity.distance = entity_data.distance().unwrap_or(0);
            slvs_entity.param
        };

        entity_data
            .param_vals()
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                val.map(|val| self.update_param(param_h[i], entity_data.group(), val))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entity_data)
    }

    /// Update the constraint data within the system, and return the updated constraint data.
    ///
    /// # Arguments
    ///
    /// * `constraint_handle` - Handle for the constraint you want to update.
    /// * `f` - Function that takes a reference to the constraint data and mutates it.
    ///
    /// # Errors
    ///
    /// Returns an error if specified constraint does not exist, or if the constraint data
    /// references any entity that does not exist in the system.
    pub fn update_constraint<C, F>(
        &mut self,
        constraint_handle: &ConstraintHandle<C>,
        f: F,
    ) -> Result<C, &'static str>
    where
        C: AsConstraintData,
        F: FnOnce(&mut C),
    {
        let mut constraint_data = self.constraint_data(constraint_handle)?;
        f(&mut constraint_data);
        self.constraint_target(&constraint_data)?;

        let slvs_constraint = self.mut_slvs_constraint(constraint_handle.handle())?;
        slvs_constraint.group = constraint_data.group();
        slvs_constraint.wrkpl = constraint_data.workplane().unwrap_or(SLVS_FREE_IN_3D);
        slvs_constraint.valA = constraint_data.val().unwrap_or(0.0);

        let [pt_a, pt_b] = constraint_data.points().unwrap_or([0; 2]);
        slvs_constraint.ptA = pt_a;
        slvs_constraint.ptB = pt_b;

        let [entity_a, entity_b, entity_c, entity_d] = constraint_data.entities().unwrap_or([0; 4]);

        slvs_constraint.entityA = entity_a;
        slvs_constraint.entityB = entity_b;
        slvs_constraint.entityC = entity_c;
        slvs_constraint.entityD = entity_d;

        let [other, other2] = constraint_data.others();
        slvs_constraint.other = other as _;
        slvs_constraint.other2 = other2 as _;

        Ok(constraint_data)
    }

    /// Deletes a group from the system
    ///
    /// Note that this *will not* delete the entities and constraints that belong to
    /// said group.
    ///
    /// # Arguments
    ///
    /// * `group` - The group to be deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::System;
    ///
    /// let mut sys = System::new();
    ///
    /// let g1 = sys.add_group();
    /// let g2 = sys.add_group();
    ///
    /// sys.delete_group(g1).expect("g1 deleted");
    /// let groups = sys.groups();
    /// assert!(!groups.contains(&g1));
    /// assert!(groups.contains(&g2));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if specified group does not exist in system.
    pub fn delete_group(&mut self, group: Group) -> Result<Group, &'static str> {
        let ix = self.group_ix(group.handle())?;
        self.groups.list.remove(ix);

        Ok(group)
    }

    /// Deletes an entity from the system, and returns the data for that entity.
    ///
    /// Note that this *will not* delete entities and constraints that reference
    /// the deleted entity.
    ///
    /// # Arguments
    ///
    /// * `entity_handle` - Handle for entity to be deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{
    /// entity::{LineSegment, Point},
    /// System,
    /// };
    ///
    /// let mut sys = System::new();
    /// let g1 = sys.add_group();
    ///
    /// let p1 = sys
    ///     .sketch(Point::new_in_3d(g1, [0.0, 0.0, 0.0]))
    ///     .expect("p1 created");
    /// let p2 = sys
    ///     .sketch(Point::new_in_3d(g1, [10.0, 10.0, 10.0]))
    ///     .expect("p2 created");
    /// let line = sys
    ///     .sketch(LineSegment::new(g1, p1, p2))
    ///     .expect("line created between p1 and p2");
    ///
    /// sys.delete_entity(p2)
    ///     .expect("p2 is deleted from the system");
    /// let entity_handles = sys.entity_handles(None, None);
    /// assert!(entity_handles.iter().any(|e| Ok(p1) == e.try_into()));
    /// assert!(!entity_handles.iter().any(|e| Ok(p2) == e.try_into()));
    /// assert!(entity_handles.iter().any(|e| Ok(line) == e.try_into()));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if specified entity does not exist in system.
    pub fn delete_entity<E: AsEntityData>(
        &mut self,
        entity_handle: EntityHandle<E>,
    ) -> Result<E, &'static str> {
        let entity_data = self.entity_data(&entity_handle)?;

        let ix = self.entity_ix(entity_handle.handle())?;
        let deleted_entity = self.entities.list.remove(ix);

        for param_h in deleted_entity.param {
            if param_h != 0 {
                self.delete_param(param_h)?
            }
        }

        Ok(entity_data)
    }

    /// Deletes a constraint from the system, and returns the data for that constraint.
    ///
    /// # Arguments
    ///
    /// * `constraint_handle` - Handle for the constraint to be deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if specified constraint does not exist in system.
    pub fn delete_constraint<C: AsConstraintData>(
        &mut self,
        constraint_handle: ConstraintHandle<C>,
    ) -> Result<C, &'static str> {
        let constraint_data = self.constraint_data(&constraint_handle)?;

        let ix = self.constraint_ix(constraint_handle.handle())?;
        self.constraints.list.remove(ix);

        Ok(constraint_data)
    }

    /// Sets an entity to be dragged.
    ///
    /// This tells the solver that it should attempt to change this entity location
    /// as little as possible, even if it requires other parameters to be changed more.
    ///
    /// This property can be cleared by calling [`System::clear_dragged`].
    ///
    /// # Arguments
    ///
    /// * `entity_handle` - The entity to be set to dragged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slvs::{constraint::PtPtDistance, entity::Point, System};
    ///
    /// let mut sys = System::new();
    /// let g = sys.add_group();
    ///
    /// let p1 = sys
    ///     .sketch(Point::new_in_3d(g, [0.0, 0.0, 0.0]))
    ///     .expect("p1 created");
    /// let p2 = sys
    ///     .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
    ///     .expect("p2 created");
    /// sys.constrain(PtPtDistance::new(g, p1, p2, 100.0, None))
    ///     .expect("p1 and p2 are 100 units apart");
    ///
    /// sys.set_dragged(&p1)
    ///     .expect("Try not to move p1 when solving");
    /// sys.solve(&g);
    /// sys.clear_dragged();
    ///
    /// println!(
    ///     "{:#?}",
    ///     sys.entity_data(&p2)
    ///         .expect("p1 should still be close to (0, 0, 0)")
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if specified entity does not exist, or if entity references
    /// other entities that do not exist.
    pub fn set_dragged<E: AsEntityData>(
        &mut self,
        entity_handle: &EntityHandle<E>,
    ) -> Result<(), &'static str> {
        let slvs_entity = self.slvs_entity(entity_handle.handle())?;

        self.dragged = match slvs_entity.type_ as _ {
            SLVS_E_ARC_OF_CIRCLE => self.slvs_entity(slvs_entity.point[0])?.param,
            SLVS_E_CIRCLE => self.slvs_entity(slvs_entity.distance)?.param,
            SLVS_E_CUBIC => self.slvs_entity(slvs_entity.point[0])?.param,
            SLVS_E_DISTANCE => slvs_entity.param,
            SLVS_E_LINE_SEGMENT => self.slvs_entity(slvs_entity.point[0])?.param,
            SLVS_E_NORMAL_IN_2D => {
                self.slvs_entity(self.slvs_entity(slvs_entity.wrkpl)?.normal)?
                    .param
            }
            SLVS_E_NORMAL_IN_3D => slvs_entity.param,
            SLVS_E_POINT_IN_2D | SLVS_E_POINT_IN_3D => slvs_entity.param,
            SLVS_E_WORKPLANE => self.slvs_entity(slvs_entity.normal)?.param,
            _ => panic!("Unknown Slvs_Entity type value {}", slvs_entity.type_),
        };

        Ok(())
    }

    /// Clear the dragged entity.
    ///
    /// See [`System::set_dragged`] for more information.
    pub fn clear_dragged(&mut self) {
        self.dragged = [0; 4];
    }

    /// Solve the geometric constraint.
    ///
    /// The solver will attempt to satistfy the constraints within the specified group
    /// to within tolerance.
    ///
    /// There are three possible outcomes for the solver.
    ///
    /// * All constraints were satisfied to within numerical tolerance, in which
    /// case [`SolveResult::Ok`] is returned.
    /// * The solver can prove that two constraints are inconsistent. In that case, a
    /// list of inconsistent results are included in [`SolveResult::Fail`].
    /// * The solver cannot prove that two constraints are inconsistent, but it cannot
    /// find a solution. The list of unsatisfied constraints are included in [`SolveResult::Fail`].
    ///
    /// # Arguments
    ///
    /// * `group` - Only entities within this group are modified during solve.
    pub fn solve(&mut self, group: &Group) -> SolveResult {
        let _lock = SolverGuard::lock();

        let mut failed_handles: Vec<Slvs_hConstraint> = vec![0; self.constraints.list.len()];
        let mut slvs_system = Slvs_System::from(self, &mut failed_handles);

        unsafe {
            Slvs_Solve(&mut slvs_system, group.handle());
        };

        match slvs_system.result as _ {
            SLVS_RESULT_OKAY => SolveResult::Ok {
                dof: slvs_system.dof,
            },
            _ => SolveResult::Fail {
                dof: slvs_system.dof,
                reason: slvs_system.result.into(),
                failed_constraints: failed_handles
                    .into_iter()
                    .filter_map(|h| match h {
                        0 => None,
                        _ => Some(self.boxed_constraint_handle(*self.slvs_constraint(h).unwrap())),
                    })
                    .collect(),
            },
        }
    }
}

struct SolverGuard;

impl SolverGuard {
    fn lock() -> MutexGuard<'static, ()> {
        static SOLVER_MUTEX: Mutex<()> = Mutex::new(());
        SOLVER_MUTEX.lock().unwrap()
    }
}

/// The solver will converge all parameter values to within this tolerance.
///
/// This is a hard-coded value, and not configurable by the user.
pub const SOLVE_TOLERANCE: f64 = 10e-6;

/// Information on the results of [`System::solve`].
#[derive(Debug)]
pub enum SolveResult {
    /// Solver was able to find a solution to the constraints within tolerance.
    ///
    /// Note that this *does not* mean that the system is fully constrained. Just
    /// that all constraints are satisfied.
    Ok {
        /// The number of unconstrained degrees of freedom.
        dof: i32,
    },

    /// Solver was unable to find a solution that satisfies all constraints.
    Fail {
        /// The number of unconstrained degrees of freedom.
        dof: i32,
        /// Reason for the failure.
        reason: FailReason,
        /// Constraints that were inconsistent or unsatisfied during the solve step.
        failed_constraints: Vec<Box<dyn AsConstraintHandle>>,
    },
}

/// Reasons that the constraint solver failed.
///
/// # Examples
///
/// Most commonly, a failed solve will result from an over-constrained system. This can
/// fall into two different categories: consistently over-constrained and inconsistently
/// over-constrained.
///
/// A **consistently over-constrained** system is solveable, but has redundant constraints.
/// In the example below, the distance between `p1` and `p2` is constrained to be
/// 10 units *twice*.
///
/// Note that while the solver returns a failure, the locations for `p1` and `p2` have
/// moved to satisfy the constraint.
/// ```
/// use slvs::{
/// constraint::PtPtDistance,
/// entity::Point,
/// system::{FailReason, SolveResult},
/// System,
/// };
///
///     let mut sys = System::new();
/// let g = sys.add_group();
///
/// let p1 = sys
///     .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
///     .expect("p1 created");
/// let p2 = sys
///     .sketch(Point::new_in_3d(g, [20.0, 20.0, 20.0]))
///     .expect("p2 created");
///
/// // distance between p1 and p2 is 10
/// sys.constrain(PtPtDistance::new(g, p1, p2, 10.0, None))
///     .expect("distance constraint added");
/// // distance between p1 and p2 is 10, a second time
/// sys.constrain(PtPtDistance::new(g, p1, p2, 10.0, None))
///     .expect("distance constraint added");
///
/// let solve_result = sys.solve(&g);
///
/// if let SolveResult::Fail { reason, .. } = solve_result {
///     assert_eq!(reason, FailReason::Inconsistent);
///     println!("{:#?}", sys.entity_data(&p1));
///     println!("{:#?}", sys.entity_data(&p2));
/// }
/// ```
///
/// An **inconsistently over-constrained* system has constraints that cannot be satisfied
/// simultaneously. Here, we have tried to constrain the distances between `p1` and `p2`
/// to be 10 units and 20 units apart, at the same time.
///
/// With inconsistently over-constrained systems, entities will remain in their initial
/// position after the solve attempt.
/// ```
/// use slvs::{
/// constraint::PtPtDistance,
/// entity::Point,
/// system::{FailReason, SolveResult},
/// System,
/// };
///
///     let mut sys = System::new();
/// let g = sys.add_group();
///
/// let p1 = sys
///     .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
///     .expect("p1 created");
/// let p2 = sys
///     .sketch(Point::new_in_3d(g, [20.0, 20.0, 20.0]))
///     .expect("p2 created");
///
/// // distance between p1 and p2 is 10
/// sys.constrain(PtPtDistance::new(g, p1, p2, 10.0, None))
///     .expect("distance constraint added");
/// // distance between p1 and p2 is 20
/// sys.constrain(PtPtDistance::new(g, p1, p2, 20.0, None))
///     .expect("distance constraint added");
///
/// let solve_result = sys.solve(&g);
///
/// if let SolveResult::Fail { reason, .. } = solve_result {
///     assert_eq!(reason, FailReason::Inconsistent);
///     println!("{:#?}", sys.entity_data(&p1));
///     println!("{:#?}", sys.entity_data(&p2));
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FailReason {
    Inconsistent,
    /// The conditions required to ensure that [Newton's method](https://en.wikipedia.org/wiki/Newton's_method)
    /// will converge were not met.
    DidntConverge,
    /// The system exceeds the hard-coded maximum of 2048 variables.
    TooManyUnknowns,
}

impl From<i32> for FailReason {
    fn from(value: i32) -> Self {
        match value as _ {
            SLVS_RESULT_INCONSISTENT => Self::Inconsistent,
            SLVS_RESULT_DIDNT_CONVERGE => Self::DidntConverge,
            SLVS_RESULT_TOO_MANY_UNKNOWNS => Self::TooManyUnknowns,
            _ => panic!("unknown result value: {}", value),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Private methods for interfacing with slvs elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub(crate) fn group_ix(&self, h: Slvs_hGroup) -> Result<usize, &'static str> {
        self.groups
            .list
            .binary_search_by_key(&h, |g| g.handle())
            .map_err(|_| "Specified group not found.")
    }

    pub(crate) fn param_ix(&self, h: Slvs_hParam) -> Result<usize, &'static str> {
        self.params
            .list
            .binary_search_by_key(&h, |&Slvs_Param { h, .. }| h)
            .map_err(|_| "Specified parameter not found.")
    }

    pub(crate) fn add_param(&mut self, group: Slvs_hGroup, val: f64) -> Slvs_hParam {
        let new_param = Slvs_Param {
            h: self.params.next_h(),
            group,
            val,
        };

        self.params.list.push(new_param);
        self.params.list.last().unwrap().h
    }

    pub(crate) fn update_param(
        &mut self,
        h: Slvs_hParam,
        group: Slvs_hGroup,
        val: f64,
    ) -> Result<(), &'static str> {
        let mut param = self.mut_slvs_param(h)?;
        param.group = group;
        param.val = val;

        Ok(())
    }

    pub(crate) fn delete_param(&mut self, h: Slvs_hParam) -> Result<(), &'static str> {
        let ix = self.param_ix(h)?;
        self.params.list.remove(ix);

        Ok(())
    }

    pub(crate) fn slvs_param(&self, h: Slvs_hParam) -> Result<&Slvs_Param, &'static str> {
        let ix = self.param_ix(h)?;
        Ok(&self.params.list[ix])
    }

    pub(crate) fn mut_slvs_param(
        &mut self,
        h: Slvs_hParam,
    ) -> Result<&mut Slvs_Param, &'static str> {
        let ix = self.param_ix(h)?;
        Ok(&mut self.params.list[ix])
    }

    pub(crate) fn entity_ix(&self, h: Slvs_hEntity) -> Result<usize, &'static str> {
        self.entities
            .list
            .binary_search_by_key(&h, |&Slvs_Entity { h, .. }| h)
            .map_err(|_| "Specified entity not found.")
    }

    pub(crate) fn slvs_entity(&self, h: Slvs_hEntity) -> Result<&Slvs_Entity, &'static str> {
        let ix = self.entity_ix(h)?;
        Ok(&self.entities.list[ix])
    }

    pub(crate) fn mut_slvs_entity(
        &mut self,
        h: Slvs_hEntity,
    ) -> Result<&mut Slvs_Entity, &'static str> {
        let ix = self.entity_ix(h)?;
        Ok(&mut self.entities.list[ix])
    }

    pub(crate) fn sketch_target<E: AsEntityData>(
        &self,
        entity_data: &E,
    ) -> Result<Option<Slvs_hEntity>, &'static str> {
        let mut referenced_workplanes = Vec::new();

        if let Some(points_h) = entity_data.points() {
            let slvs_points: Result<Vec<_>, _> = points_h
                .iter()
                .filter_map(|point_h| match point_h {
                    0 => None,
                    _ => Some(self.slvs_entity(*point_h)),
                })
                .collect();
            referenced_workplanes.extend(slvs_points?.iter().map(|slvs_point| slvs_point.wrkpl));
        }

        if let Some(normal_h) = entity_data.normal() {
            let slvs_normal = self.slvs_entity(normal_h)?;
            referenced_workplanes.push(slvs_normal.wrkpl);
        }

        if referenced_workplanes.is_empty() {
            Ok(entity_data.workplane())
        } else if let Some(workplane_h) = entity_data.workplane() {
            if referenced_workplanes.iter().all(|x| *x == workplane_h) {
                Ok(Some(workplane_h))
            } else {
                Err("Referenced points should all lie on workplane")
            }
        } else if referenced_workplanes
            .iter()
            .all(|x| *x == referenced_workplanes[0])
        {
            Ok(Some(referenced_workplanes[0]))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn constraint_ix(&self, h: Slvs_hConstraint) -> Result<usize, &'static str> {
        self.constraints
            .list
            .binary_search_by_key(&h, |&Slvs_Constraint { h, .. }| h)
            .map_err(|_| "Specified constraint not found.")
    }

    pub(crate) fn slvs_constraint(
        &self,
        h: Slvs_hConstraint,
    ) -> Result<&Slvs_Constraint, &'static str> {
        let ix = self.constraint_ix(h)?;
        Ok(&self.constraints.list[ix])
    }

    pub(crate) fn mut_slvs_constraint(
        &mut self,
        h: Slvs_hConstraint,
    ) -> Result<&mut Slvs_Constraint, &'static str> {
        let ix = self.constraint_ix(h)?;
        Ok(&mut self.constraints.list[ix])
    }

    pub(crate) fn constraint_target<C: AsConstraintData>(
        &self,
        constraint_data: &C,
    ) -> Result<Option<Slvs_hEntity>, &'static str> {
        let mut referenced_workplanes = Vec::new();

        if let Some(points_h) = constraint_data.points() {
            let slvs_points: Result<Vec<_>, _> = points_h
                .iter()
                .filter_map(|point_h| match point_h {
                    0 => None,
                    _ => Some(self.slvs_entity(*point_h)),
                })
                .collect();
            referenced_workplanes.extend(slvs_points?.iter().map(|slvs_point| slvs_point.wrkpl));
        }

        if let Some(entities_h) = constraint_data.entities() {
            let slvs_points: Result<Vec<_>, _> = entities_h
                .iter()
                .filter_map(|entity_h| match entity_h {
                    0 => None,
                    _ => Some(self.slvs_entity(*entity_h)),
                })
                .collect();
            referenced_workplanes.extend(slvs_points?.iter().map(|slvs_point| slvs_point.wrkpl));
        }

        if let Some(workplane_h) = constraint_data.workplane() {
            Ok(Some(workplane_h))
        } else if referenced_workplanes
            .iter()
            .all(|x| *x == referenced_workplanes[0])
        {
            Ok(Some(referenced_workplanes[0]))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn boxed_constraint_handle(
        &self,
        slvs_constraint: Slvs_Constraint,
    ) -> Box<dyn AsConstraintHandle> {
        match slvs_constraint.type_ as _ {
            SLVS_C_CURVE_CURVE_TANGENT => {
                let slvs_curve_a = self.slvs_entity(slvs_constraint.entityA).unwrap();
                let slvs_curve_b = self.slvs_entity(slvs_constraint.entityB).unwrap();

                match (slvs_curve_a.type_ as _, slvs_curve_b.type_ as _) {
                    (SLVS_E_ARC_OF_CIRCLE, SLVS_E_ARC_OF_CIRCLE) => {
                        Box::new(ConstraintHandle::<
                            CurveCurveTangent<ArcOfCircle, ArcOfCircle>,
                        >::new(slvs_constraint.h))
                            as Box<dyn AsConstraintHandle>
                    }
                    (SLVS_E_ARC_OF_CIRCLE, SLVS_E_CUBIC) => Box::new(ConstraintHandle::<
                        CurveCurveTangent<ArcOfCircle, Cubic>,
                    >::new(
                        slvs_constraint.h
                    ))
                        as Box<dyn AsConstraintHandle>,
                    (SLVS_E_CUBIC, SLVS_E_ARC_OF_CIRCLE) => Box::new(ConstraintHandle::<
                        CurveCurveTangent<Cubic, ArcOfCircle>,
                    >::new(
                        slvs_constraint.h
                    ))
                        as Box<dyn AsConstraintHandle>,
                    (SLVS_E_CUBIC, SLVS_E_CUBIC) => {
                        Box::new(ConstraintHandle::<CurveCurveTangent<Cubic, Cubic>>::new(
                            slvs_constraint.h,
                        )) as Box<dyn AsConstraintHandle>
                    }
                    _ => panic!("SLVS_C_CURVE_CURVE_TANGENT should reference two curves."),
                }
            }
            SLVS_C_DIAMETER => {
                let slvs_radius = self.slvs_entity(slvs_constraint.entityA).unwrap();

                match slvs_radius.type_ as _ {
                    SLVS_E_ARC_OF_CIRCLE => Box::new(
                        ConstraintHandle::<Diameter<ArcOfCircle>>::new(slvs_constraint.h),
                    ) as Box<dyn AsConstraintHandle>,
                    SLVS_E_CIRCLE => {
                        Box::new(ConstraintHandle::<Diameter<Circle>>::new(slvs_constraint.h))
                            as Box<dyn AsConstraintHandle>
                    }
                    _ => panic!("SLVS_C_DIAMETER should reference arcs."),
                }
            }
            SLVS_C_EQUAL_RADIUS => {
                let slvs_radius_a = self.slvs_entity(slvs_constraint.entityA).unwrap();
                let slvs_radius_b = self.slvs_entity(slvs_constraint.entityB).unwrap();

                match (slvs_radius_a.type_ as _, slvs_radius_b.type_ as _) {
                    (SLVS_E_ARC_OF_CIRCLE, SLVS_E_ARC_OF_CIRCLE) => Box::new(ConstraintHandle::<
                        EqualRadius<ArcOfCircle, ArcOfCircle>,
                    >::new(
                        slvs_constraint.h
                    ))
                        as Box<dyn AsConstraintHandle>,
                    (SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE) => {
                        Box::new(ConstraintHandle::<EqualRadius<ArcOfCircle, Circle>>::new(
                            slvs_constraint.h,
                        )) as Box<dyn AsConstraintHandle>
                    }
                    (SLVS_E_CIRCLE, SLVS_E_ARC_OF_CIRCLE) => {
                        Box::new(ConstraintHandle::<EqualRadius<Circle, ArcOfCircle>>::new(
                            slvs_constraint.h,
                        )) as Box<dyn AsConstraintHandle>
                    }
                    (SLVS_E_CIRCLE, SLVS_E_CIRCLE) => {
                        Box::new(ConstraintHandle::<EqualRadius<Circle, Circle>>::new(
                            slvs_constraint.h,
                        )) as Box<dyn AsConstraintHandle>
                    }
                    _ => panic!("SLVS_C_EQUAL_RADIUS should reference two curves."),
                }
            }
            SLVS_C_PROJ_PT_DISTANCE => {
                let slvs_line = self.slvs_entity(slvs_constraint.entityA).unwrap();

                match slvs_line.type_ as _ {
                    SLVS_E_LINE_SEGMENT => Box::new(
                        ConstraintHandle::<ProjPtDistance<LineSegment>>::new(slvs_constraint.h),
                    ) as Box<dyn AsConstraintHandle>,

                    SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => {
                        Box::new(ConstraintHandle::<ProjPtDistance<Normal>>::new(
                            slvs_constraint.h,
                        )) as Box<dyn AsConstraintHandle>
                    }
                    _ => panic!("SLVS_C_EQUAL_RADIUS should reference a line or normal."),
                }
            }
            SLVS_C_PT_ON_CIRCLE => {
                let slvs_radius = self.slvs_entity(slvs_constraint.entityA).unwrap();

                match slvs_radius.type_ as _ {
                    SLVS_E_ARC_OF_CIRCLE => Box::new(
                        ConstraintHandle::<PtOnCircle<ArcOfCircle>>::new(slvs_constraint.h),
                    ) as Box<dyn AsConstraintHandle>,
                    SLVS_E_CIRCLE => Box::new(ConstraintHandle::<PtOnCircle<Circle>>::new(
                        slvs_constraint.h,
                    )) as Box<dyn AsConstraintHandle>,
                    _ => panic!("SLVS_C_EQUAL_RADIUS should reference an arc or circle."),
                }
            }
            _ => slvs_constraint.into(),
        }
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}
