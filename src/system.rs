use serde::Serialize;

use crate::{
    bindings::{
        Slvs_Constraint, Slvs_Entity, Slvs_Param, Slvs_Solve, Slvs_System, Slvs_hConstraint,
        Slvs_hEntity, Slvs_hGroup, Slvs_hParam, SLVS_C_CURVE_CURVE_TANGENT, SLVS_C_DIAMETER,
        SLVS_C_EQUAL_RADIUS, SLVS_C_PROJ_PT_DISTANCE, SLVS_C_PT_ON_CIRCLE, SLVS_E_ARC_OF_CIRCLE,
        SLVS_E_CIRCLE, SLVS_E_CUBIC, SLVS_E_LINE_SEGMENT, SLVS_E_NORMAL_IN_2D, SLVS_E_NORMAL_IN_3D,
        SLVS_FREE_IN_3D,
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
    solver::{FailReason, SolveFail, SolveOkay},
};

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

#[derive(Debug, Serialize)]
pub struct System {
    pub groups: Elements<Group>,
    pub params: Elements<Slvs_Param>,
    pub entities: Elements<Slvs_Entity>,
    pub constraints: Elements<Slvs_Constraint>,
    pub calculate_faileds: bool,
    pub dragged: [Slvs_hParam; 4],
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

////////////////////////////////////////////////////////////////////////////////
// Adding elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn add_group(&mut self) -> Group {
        let new_group = Group(self.groups.get_next_h());

        self.groups.list.push(new_group);
        self.groups.list.last().cloned().unwrap()
    }

    pub fn sketch<E: AsEntityData>(
        &mut self,
        entity_data: E,
    ) -> Result<EntityHandle<E>, &'static str> {
        let workplane_h = self.sketch_target(&entity_data)?;

        // ArcOfCircle requires a Normal, which is identical to its workplane's normal
        let normal_h = if SLVS_E_ARC_OF_CIRCLE == entity_data.slvs_type() as _ {
            let slvs_workplane = self.slvs_entity(entity_data.workplane().unwrap())?;
            (*slvs_workplane).normal
        } else {
            entity_data.normal().unwrap_or(0)
        };

        let param_h = entity_data.param_vals().map(|val| match val {
            Some(val) => self.add_param(entity_data.group(), val),
            None => 0,
        });

        let slvs_entity = Slvs_Entity {
            h: self.entities.get_next_h(),
            group: entity_data.group(),
            type_: entity_data.slvs_type(),
            wrkpl: workplane_h.unwrap_or(SLVS_FREE_IN_3D),
            point: entity_data.points().unwrap_or([0, 0, 0, 0]),
            normal: normal_h,
            distance: entity_data.distance().unwrap_or(0),
            param: param_h,
        };

        self.entities.list.push(slvs_entity);

        let entity_handle = EntityHandle::new(slvs_entity.h);

        Ok(entity_handle)
    }

    pub fn constrain<C: AsConstraintData>(
        &mut self,
        constraint_data: C,
    ) -> Result<ConstraintHandle<C>, &'static str> {
        self.validate_constraint_data(&constraint_data)?;

        let mut new_slvs_constraint = Slvs_Constraint::new(
            self.constraints.get_next_h(),
            constraint_data.group(),
            constraint_data.slvs_type(),
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

        self.constraints.list.push(new_slvs_constraint);

        let constraint_handle = ConstraintHandle::new(new_slvs_constraint.h);

        Ok(constraint_handle)
    }

    // Private as user has no reason to create bare param without linking to an entity.
    pub(crate) fn add_param(&mut self, group: Slvs_hGroup, val: f64) -> Slvs_hParam {
        let new_param = Slvs_Param {
            h: self.params.get_next_h(),
            group,
            val,
        };

        self.params.list.push(new_param);
        self.params.list.last().unwrap().h
    }
}

////////////////////////////////////////////////////////////////////////////////
// Getting element data
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn groups(&self) -> Vec<Group> {
        self.groups.list.clone()
    }

    pub fn entity_handles(&self, group: Option<&Group>) -> Vec<Box<dyn AsEntityHandle>> {
        self.entities
            .list
            .iter()
            .filter(|&slvs_entity| group.map_or(true, |group| slvs_entity.group == group.handle()))
            .map(|&slvs_entity| slvs_entity.into())
            .collect()
    }

    pub fn entity_data<E: AsEntityData>(
        &self,
        entity_handle: &EntityHandle<E>,
    ) -> Result<E, &'static str> {
        E::from_system(self, entity_handle)
    }

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
                    [slvs_constraint.ptA, slvs_constraint.ptB].contains(&entity_handle.handle())
                })
            })
            .map(|&slvs_constraint| match slvs_constraint.type_ as _ {
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
                    }
                }
                SLVS_C_DIAMETER => {
                    let slvs_radius = self.slvs_entity(slvs_constraint.entityA).unwrap();

                    match slvs_radius.type_ as _ {
                        SLVS_E_ARC_OF_CIRCLE => Box::new(
                            ConstraintHandle::<Diameter<ArcOfCircle>>::new(slvs_constraint.h),
                        )
                            as Box<dyn AsConstraintHandle>,
                        SLVS_E_CIRCLE => {
                            Box::new(ConstraintHandle::<Diameter<Circle>>::new(slvs_constraint.h))
                                as Box<dyn AsConstraintHandle>
                        }
                    }
                }
                SLVS_C_EQUAL_RADIUS => {
                    let slvs_radius_a = self.slvs_entity(slvs_constraint.entityA).unwrap();
                    let slvs_radius_b = self.slvs_entity(slvs_constraint.entityB).unwrap();

                    match (slvs_radius_a.type_ as _, slvs_radius_b.type_ as _) {
                        (SLVS_E_ARC_OF_CIRCLE, SLVS_E_ARC_OF_CIRCLE) => Box::new(
                            ConstraintHandle::<EqualRadius<ArcOfCircle, ArcOfCircle>>::new(
                                slvs_constraint.h,
                            ),
                        )
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
                    }
                }
                SLVS_C_PROJ_PT_DISTANCE => {
                    let slvs_line = self.slvs_entity(slvs_constraint.entityA).unwrap();

                    match slvs_line.type_ as _ {
                        SLVS_E_LINE_SEGMENT => {
                            Box::new(ConstraintHandle::<ProjPtDistance<LineSegment>>::new(
                                slvs_constraint.h,
                            )) as Box<dyn AsConstraintHandle>
                        }

                        SLVS_E_NORMAL_IN_2D | SLVS_E_NORMAL_IN_3D => {
                            Box::new(ConstraintHandle::<ProjPtDistance<Normal>>::new(
                                slvs_constraint.h,
                            )) as Box<dyn AsConstraintHandle>
                        }
                    }
                }
                SLVS_C_PT_ON_CIRCLE => {
                    let slvs_radius = self.slvs_entity(slvs_constraint.entityA).unwrap();

                    match slvs_radius.type_ as _ {
                        SLVS_E_ARC_OF_CIRCLE => Box::new(
                            ConstraintHandle::<PtOnCircle<ArcOfCircle>>::new(slvs_constraint.h),
                        )
                            as Box<dyn AsConstraintHandle>,
                        SLVS_E_CIRCLE => Box::new(ConstraintHandle::<PtOnCircle<Circle>>::new(
                            slvs_constraint.h,
                        )) as Box<dyn AsConstraintHandle>,
                    }
                }
                _ => slvs_constraint.into(),
            })
            .collect()
    }

    pub fn constraint_data<C: AsConstraintData>(
        &self,
        constraint_handle: &ConstraintHandle<C>,
    ) -> Result<C, &'static str> {
        C::from_system(self, constraint_handle)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Updating element data
////////////////////////////////////////////////////////////////////////////////

impl System {
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

    pub fn update_entity<E, F>(
        &mut self,
        entity_handle: &EntityHandle<E>,
        f: F,
    ) -> Result<E, &'static str>
    where
        E: AsEntityData,
        F: FnOnce(&mut E),
    {
        let slvs_entity = self.mut_slvs_entity(entity_handle.handle())?;

        let mut entity_data = self.entity_data(entity_handle)?;
        f(&mut entity_data);

        let workplane_h = self.sketch_target(&entity_data)?;
        slvs_entity.wrkpl = workplane_h.unwrap_or(SLVS_FREE_IN_3D);

        slvs_entity.point = entity_data.points().unwrap_or([0, 0, 0, 0]);

        // ArcOfCircle requires a Normal, which is identical to its workplane's normal
        let normal_h = if SLVS_E_ARC_OF_CIRCLE == entity_data.slvs_type() as _ {
            let slvs_workplane = self.slvs_entity(entity_data.workplane().unwrap())?;
            (*slvs_workplane).normal
        } else {
            entity_data.normal().unwrap_or(0)
        };
        slvs_entity.normal = normal_h;

        slvs_entity.distance = entity_data.distance().unwrap_or(0);

        entity_data
            .param_vals()
            .iter()
            .enumerate()
            .for_each(|(i, Some(val))| {
                self.update_param(slvs_entity.param[i], entity_data.group(), *val);
            });

        Ok(entity_data)
    }

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
        self.validate_constraint_data(&constraint_data)?;

        let slvs_constraint = self
            .mut_slvs_constraint(constraint_handle.handle())
            .unwrap();
        slvs_constraint.set_group(constraint_data.group());

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
// Deleting elements
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn delete_group(&mut self, group: Group) -> Result<Group, &'static str> {
        let ix = self.group_ix(group.handle())?;
        self.groups.list.remove(ix);

        Ok(group)
    }

    pub(crate) fn delete_param(&mut self, h: Slvs_hParam) -> Result<(), &'static str> {
        let ix = self.param_ix(h)?;
        self.params.list.remove(ix);

        Ok(())
    }

    pub fn delete_entity<E: AsEntityData>(
        &mut self,
        entity_handle: EntityHandle<E>,
    ) -> Result<E, &'static str> {
        let entity_data = self.entity_data(&entity_handle)?;

        let ix = self.entity_ix(entity_handle.handle())?;
        let deleted_entity = self.entities.list.remove(ix);

        for param_h in deleted_entity.param {
            self.delete_param(param_h)?
        }

        Ok(entity_data)
    }

    pub fn delete_constraint<C: AsConstraintData>(
        &mut self,
        constraint_handle: ConstraintHandle<C>,
    ) -> Result<C, &'static str> {
        let constraint_data = self.constraint_data(&constraint_handle)?;

        let ix = self.constraint_ix(constraint_handle.handle())?;
        self.constraints.list.remove(ix);

        Ok(constraint_data)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Solving the system
////////////////////////////////////////////////////////////////////////////////

impl System {
    // Check generate.cpp. Has info on mapping from entity to paramater
    pub fn set_dragged(&mut self, entity_handle: &EntityHandle<impl AsEntityData>) {
        if let Ok(slvs_entity) = self.slvs_entity(entity_handle.handle()) {
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
            Slvs_Solve(&mut slvs_system, group.handle());
        };

        match FailReason::try_from(slvs_system.result) {
            Ok(fail_reason) => Err(SolveFail {
                dof: slvs_system.dof,
                reason: fail_reason,
                failed_constraints: failed_handles.into_iter().filter(|&h| h != 0).collect(),
            }),
            Err(_) => Ok(SolveOkay {
                dof: slvs_system.dof,
            }),
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

    pub(crate) fn entity_on_workplane(
        &self,
        h: Slvs_hEntity,
        workplane: Slvs_hEntity,
    ) -> Result<(), &'static str> {
        let slvs_entity = self.slvs_entity(h)?;

        match slvs_entity.type_ as _ {
            SLVS_E_NORMAL_IN_3D => match slvs_entity.h == self.slvs_entity(workplane)?.normal {
                true => Ok(()),
                false => Err("Normal in 3d does not match workplane's normal."),
            },
            _ => match slvs_entity.wrkpl == workplane {
                true => Ok(()),
                false => Err("Entity not on expected workplane."),
            },
        }
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
        let slvs_workplane = entity_data
            .workplane()
            .map(|h| self.slvs_entity(h))
            .transpose()?;

        let mut referenced_workplanes = Vec::new();

        let slvs_points = entity_data
            .points()
            .map(|points| {
                points
                    .iter()
                    .filter_map(|h| match h {
                        0 => None,
                        _ => Some(self.slvs_entity(*h)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        if let Some(points) = slvs_points {
            let point_workplanes: Vec<_> = points.iter().map(|point| point.h).collect();

            if let Some(workplane) = slvs_workplane {
                if !point_workplanes
                    .iter()
                    .all(|point_workplane| *point_workplane == workplane.h)
                {
                    return Err("Referenced points should all lie on workplane");
                }
            }

            referenced_workplanes.extend(point_workplanes)
        }

        let slvs_normal = entity_data
            .normal()
            .map(|h| self.slvs_entity(h))
            .transpose()?;
        if let Some(normal) = slvs_normal {
            if let Some(workplane) = slvs_workplane {
                if normal.h != workplane.h {
                    return Err("Referenced normal should lie on workplane");
                }
            }

            referenced_workplanes.push(normal.h);
        }

        let slvs_distance = entity_data
            .distance()
            .map(|h| self.slvs_entity(h))
            .transpose()?;

        if let Some(distance) = slvs_distance {
            if let Some(workplane) = slvs_workplane {
                if distance.h != workplane.h {
                    return Err("Referenced distance should lie on workplane");
                }
            }

            referenced_workplanes.push(distance.h);
        }

        match referenced_workplanes
            .iter()
            .all(|workplane| *workplane == referenced_workplanes[0])
        {
            true => Ok(Some(referenced_workplanes[0])),
            false => Ok(None),
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

    pub(crate) fn validate_constraint_data(
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
