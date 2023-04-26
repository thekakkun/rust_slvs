use std::any::Any;
use std::iter::zip;
use std::marker::PhantomData;

mod bindings;
use bindings::Slvs_hGroup;
pub use bindings::{make_quaternion, quaternion_n, quaternion_u, quaternion_v};
use bindings::{
    Slvs_Constraint, Slvs_hConstraint, SLVS_C_ANGLE, SLVS_C_ARC_ARC_DIFFERENCE,
    SLVS_C_ARC_ARC_LEN_RATIO, SLVS_C_ARC_LINE_DIFFERENCE, SLVS_C_ARC_LINE_LEN_RATIO,
    SLVS_C_ARC_LINE_TANGENT, SLVS_C_AT_MIDPOINT, SLVS_C_CUBIC_LINE_TANGENT,
    SLVS_C_CURVE_CURVE_TANGENT, SLVS_C_DIAMETER, SLVS_C_EQUAL_ANGLE, SLVS_C_EQUAL_LENGTH_LINES,
    SLVS_C_EQUAL_LINE_ARC_LEN, SLVS_C_EQUAL_RADIUS, SLVS_C_EQ_LEN_PT_LINE_D,
    SLVS_C_EQ_PT_LN_DISTANCES, SLVS_C_HORIZONTAL, SLVS_C_LENGTH_DIFFERENCE, SLVS_C_LENGTH_RATIO,
    SLVS_C_PARALLEL, SLVS_C_PERPENDICULAR, SLVS_C_POINTS_COINCIDENT, SLVS_C_PROJ_PT_DISTANCE,
    SLVS_C_PT_FACE_DISTANCE, SLVS_C_PT_IN_PLANE, SLVS_C_PT_LINE_DISTANCE, SLVS_C_PT_ON_CIRCLE,
    SLVS_C_PT_ON_FACE, SLVS_C_PT_ON_LINE, SLVS_C_PT_PLANE_DISTANCE, SLVS_C_PT_PT_DISTANCE,
    SLVS_C_SAME_ORIENTATION, SLVS_C_SYMMETRIC, SLVS_C_SYMMETRIC_HORIZ, SLVS_C_SYMMETRIC_LINE,
    SLVS_C_SYMMETRIC_VERT, SLVS_C_VERTICAL, SLVS_C_WHERE_DRAGGED,
};
use bindings::{Slvs_Entity, Slvs_hEntity, SLVS_E_NORMAL_IN_3D};
use bindings::{Slvs_Param, Slvs_hParam};
use bindings::{
    Slvs_Solve, Slvs_System, SLVS_RESULT_DIDNT_CONVERGE, SLVS_RESULT_INCONSISTENT,
    SLVS_RESULT_TOO_MANY_UNKNOWNS,
};

pub mod constraint;
use constraint::{
    AsConstraintData, Constraint, Diameter, EqualRadius, LineHorizontal, LineVertical,
    PointsHorizontal, PointsVertical, PtLineDistance, PtPtDistance,
};

mod element;
use element::{AsHandle, AsTarget, Elements};
pub use element::{Group, In3d, OnWorkplane};

pub mod entity;
use entity::{AsEntityData, Circle, Entity, FromSlvsEntity, LineSegment, Point, Workplane};

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

    pub fn sketch<E: AsEntityData>(
        &mut self,
        group: &Group,
        entity_data: E,
    ) -> Result<Entity<E>, &'static str> {
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

    pub fn constrain<C: AsConstraintData>(
        &mut self,
        group: &Group,
        constraint_data: C,
    ) -> Result<Constraint<C>, &'static str> {
        self.validate_constraint_data(&constraint_data)?;

        let mut new_slvs_constraint = Slvs_Constraint::new(
            self.constraints.get_next_h(),
            group.as_handle(),
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

        self.constraints.list.push(new_slvs_constraint);

        Ok(Constraint::new(new_slvs_constraint.h))
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
    pub fn entity_data<E, T>(&self, entity: &Entity<E>) -> Result<E, &'static str>
    where
        E: FromSlvsEntity<T>,
        T: AsTarget,
    {
        let slvs_entity = self.slvs_entity(entity.as_handle())?;
        let mut entity_data = E::from(*slvs_entity);

        let param_vals: Vec<_> = slvs_entity
            .param
            .iter()
            .filter(|&param_h| *param_h != 0)
            .filter_map(|&param_h| self.slvs_param(param_h).ok())
            .map(|&slvs_param| slvs_param.val)
            .collect();

        if !param_vals.is_empty() {
            entity_data.set_vals(param_vals);
        }

        Ok(entity_data)
    }

    // pub fn entity_data<T: AsEntityData + 'static>(
    //     &self,
    //     entity: &Entity<T>,
    // ) -> Result<T, &'static str> {
    //     self.slvs_entity(entity.as_handle()).map(|slvs_entity| {
    //         let some_entity_data: Box<dyn Any> = match slvs_entity.type_ as _ {
    //             SLVS_E_POINT_IN_3D => Box::new(Point::<In3d>::new(
    //                 self.slvs_param(slvs_entity.param[0]).unwrap().val,
    //                 self.slvs_param(slvs_entity.param[1]).unwrap().val,
    //                 self.slvs_param(slvs_entity.param[2]).unwrap().val,
    //             )),
    //             SLVS_E_POINT_IN_2D => Box::new(Point::<OnWorkplane>::new(
    //                 Entity::new(slvs_entity.wrkpl),
    //                 self.slvs_param(slvs_entity.param[0]).unwrap().val,
    //                 self.slvs_param(slvs_entity.param[1]).unwrap().val,
    //             )),
    //             SLVS_E_NORMAL_IN_3D => Box::new(Normal::<In3d>::new([
    //                 self.slvs_param(slvs_entity.param[0]).unwrap().val,
    //                 self.slvs_param(slvs_entity.param[1]).unwrap().val,
    //                 self.slvs_param(slvs_entity.param[2]).unwrap().val,
    //                 self.slvs_param(slvs_entity.param[3]).unwrap().val,
    //             ])),
    //             SLVS_E_NORMAL_IN_2D => {
    //                 Box::new(Normal::<OnWorkplane>::new(Entity::new(slvs_entity.wrkpl)))
    //             }
    //             SLVS_E_DISTANCE => match slvs_entity.wrkpl {
    //                 SLVS_FREE_IN_3D => Box::new(Distance::<In3d>::new(
    //                     self.slvs_param(slvs_entity.param[0]).unwrap().val,
    //                 )),
    //                 _ => Box::new(Distance::<OnWorkplane>::new(
    //                     Entity::new(slvs_entity.wrkpl),
    //                     self.slvs_param(slvs_entity.param[0]).unwrap().val,
    //                 )),
    //             },
    //             SLVS_E_WORKPLANE => Box::new(Workplane::new(
    //                 Entity::new(slvs_entity.point[0]),
    //                 Entity::new(slvs_entity.normal),
    //             )),
    //             SLVS_E_LINE_SEGMENT => match slvs_entity.wrkpl {
    //                 SLVS_FREE_IN_3D => Box::new(LineSegment::<In3d>::new(
    //                     Entity::new(slvs_entity.point[0]),
    //                     Entity::new(slvs_entity.point[1]),
    //                 )),
    //                 _ => Box::new(LineSegment::<OnWorkplane>::new(
    //                     Entity::new(slvs_entity.wrkpl),
    //                     Entity::new(slvs_entity.point[0]),
    //                     Entity::new(slvs_entity.point[1]),
    //                 )),
    //             },
    //             SLVS_E_CUBIC => match slvs_entity.wrkpl {
    //                 SLVS_FREE_IN_3D => Box::new(Cubic::<In3d>::new(
    //                     Entity::new(slvs_entity.point[0]),
    //                     Entity::new(slvs_entity.point[1]),
    //                     Entity::new(slvs_entity.point[2]),
    //                     Entity::new(slvs_entity.point[3]),
    //                 )),
    //                 _ => Box::new(Cubic::<OnWorkplane>::new(
    //                     Entity::new(slvs_entity.wrkpl),
    //                     Entity::new(slvs_entity.point[0]),
    //                     Entity::new(slvs_entity.point[1]),
    //                     Entity::new(slvs_entity.point[2]),
    //                     Entity::new(slvs_entity.point[3]),
    //                 )),
    //             },
    //             SLVS_E_CIRCLE => match slvs_entity.wrkpl {
    //                 SLVS_FREE_IN_3D => Box::new(Circle::<In3d>::new(
    //                     Entity::new(slvs_entity.point[0]),
    //                     Entity::new(slvs_entity.distance),
    //                     Entity::new(slvs_entity.normal),
    //                 )),
    //                 _ => Box::new(Circle::<OnWorkplane>::new(
    //                     Entity::new(slvs_entity.wrkpl),
    //                     Entity::new(slvs_entity.point[0]),
    //                     Entity::new(slvs_entity.distance),
    //                     Entity::new(slvs_entity.normal),
    //                 )),
    //             },
    //             SLVS_E_ARC_OF_CIRCLE => Box::new(ArcOfCircle::new(
    //                 Entity::new(slvs_entity.wrkpl),
    //                 Entity::new(slvs_entity.point[0]),
    //                 Entity::new(slvs_entity.point[1]),
    //                 Entity::new(slvs_entity.point[2]),
    //                 Entity::new(slvs_entity.normal),
    //             )),
    //             _ => panic!("Unknown entity type: {}", slvs_entity.type_),
    //         };

    //         *some_entity_data.downcast::<T>().unwrap()
    //     })
    // }

    pub fn constraint_data<C: AsConstraintData + 'static>(
        &self,
        constraint: &Constraint<C>,
    ) -> Result<C, &'static str> {
        self.slvs_constraint(constraint.as_handle())
            .map(|slvs_constraint| {
                let workplane = if slvs_constraint.wrkpl == 0 {
                    None
                } else {
                    Some(Entity::<Workplane>::new(slvs_constraint.wrkpl))
                };

                // TODO: This shouldn't be something I'm allowed to do...
                let some_constraint_data: Box<dyn Any> = match slvs_constraint.type_ as _ {
                    SLVS_C_POINTS_COINCIDENT => todo!(),
                    SLVS_C_PT_PT_DISTANCE => Box::new(PtPtDistance::new(
                        Entity::<Point<OnWorkplane>>::new(slvs_constraint.ptA),
                        Entity::<Point<OnWorkplane>>::new(slvs_constraint.ptB),
                        slvs_constraint.valA,
                        workplane,
                    )),
                    SLVS_C_PT_PLANE_DISTANCE => todo!(),
                    SLVS_C_PT_LINE_DISTANCE => Box::new(PtLineDistance::new(
                        Entity::<Point<OnWorkplane>>::new(slvs_constraint.ptA),
                        Entity::<LineSegment<OnWorkplane>>::new(slvs_constraint.entityA),
                        slvs_constraint.valA,
                        workplane,
                    )),
                    SLVS_C_PT_FACE_DISTANCE => todo!(),
                    SLVS_C_PT_IN_PLANE => todo!(),
                    SLVS_C_PT_ON_LINE => todo!(),
                    SLVS_C_PT_ON_FACE => todo!(),
                    SLVS_C_EQUAL_LENGTH_LINES => todo!(),
                    SLVS_C_LENGTH_RATIO => todo!(),
                    SLVS_C_EQ_LEN_PT_LINE_D => todo!(),
                    SLVS_C_EQ_PT_LN_DISTANCES => todo!(),
                    SLVS_C_EQUAL_ANGLE => todo!(),
                    SLVS_C_EQUAL_LINE_ARC_LEN => todo!(),
                    SLVS_C_SYMMETRIC => todo!(),
                    SLVS_C_SYMMETRIC_HORIZ => todo!(),
                    SLVS_C_SYMMETRIC_VERT => todo!(),
                    SLVS_C_SYMMETRIC_LINE => todo!(),
                    SLVS_C_AT_MIDPOINT => todo!(),
                    SLVS_C_HORIZONTAL => match slvs_constraint.entityA {
                        0 => Box::new(PointsHorizontal::new(
                            Entity::<Workplane>::new(slvs_constraint.wrkpl),
                            Entity::<Point<OnWorkplane>>::new(slvs_constraint.ptA),
                            Entity::<Point<OnWorkplane>>::new(slvs_constraint.ptB),
                        )),
                        _ => Box::new(LineHorizontal::new(
                            Entity::<Workplane>::new(slvs_constraint.wrkpl),
                            Entity::<LineSegment<OnWorkplane>>::new(slvs_constraint.entityA),
                        )),
                    },
                    SLVS_C_VERTICAL => match slvs_constraint.entityA {
                        0 => Box::new(PointsVertical::new(
                            Entity::<Workplane>::new(slvs_constraint.wrkpl),
                            Entity::<Point<OnWorkplane>>::new(slvs_constraint.ptA),
                            Entity::<Point<OnWorkplane>>::new(slvs_constraint.ptB),
                        )),
                        _ => Box::new(LineVertical::new(
                            Entity::<Workplane>::new(slvs_constraint.wrkpl),
                            Entity::<LineSegment<OnWorkplane>>::new(slvs_constraint.entityA),
                        )),
                    },
                    SLVS_C_DIAMETER => Box::new(Diameter::new(
                        Entity::<Circle<OnWorkplane>>::new(slvs_constraint.entityA),
                        slvs_constraint.valA,
                    )),
                    SLVS_C_PT_ON_CIRCLE => todo!(),
                    SLVS_C_SAME_ORIENTATION => todo!(),
                    SLVS_C_ANGLE => todo!(),
                    SLVS_C_PARALLEL => todo!(),
                    SLVS_C_PERPENDICULAR => todo!(),
                    SLVS_C_ARC_LINE_TANGENT => todo!(),
                    SLVS_C_CUBIC_LINE_TANGENT => todo!(),
                    SLVS_C_EQUAL_RADIUS => Box::new(EqualRadius::new(
                        Entity::<Circle<OnWorkplane>>::new(slvs_constraint.entityA),
                        Entity::<Circle<OnWorkplane>>::new(slvs_constraint.entityB),
                    )),
                    SLVS_C_PROJ_PT_DISTANCE => todo!(),
                    SLVS_C_WHERE_DRAGGED => todo!(),
                    SLVS_C_CURVE_CURVE_TANGENT => todo!(),
                    SLVS_C_LENGTH_DIFFERENCE => todo!(),
                    SLVS_C_ARC_ARC_LEN_RATIO => todo!(),
                    SLVS_C_ARC_LINE_LEN_RATIO => todo!(),
                    SLVS_C_ARC_ARC_DIFFERENCE => todo!(),
                    SLVS_C_ARC_LINE_DIFFERENCE => todo!(),

                    _ => panic!("Unknown constraint type: {}", slvs_constraint.type_),
                };

                *some_constraint_data.downcast::<C>().unwrap()
            })
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
            let slvs_entity = self.mut_slvs_entity(entity.as_handle()).unwrap();

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

    // pub fn update_entity<T, F>(&mut self, entity: &Entity<T>, f: F) -> Result<T, &'static str>
    // where
    //     T: AsEntityData + 'static,
    //     F: FnOnce(&mut T),
    // {
    //     let mut entity_data = self.entity_data(entity)?;

    //     f(&mut entity_data);
    //     self.validate_entity_data(&entity_data)?;

    //     let param_h = {
    //         let slvs_entity = self.mut_slvs_entity(entity.as_handle()).unwrap();

    //         if let Some(points) = entity_data.points() {
    //             slvs_entity.set_point(points);
    //         }
    //         if let Some(normal) = entity_data.normal() {
    //             slvs_entity.set_normal(normal);
    //         }
    //         if let Some(distance) = entity_data.distance() {
    //             slvs_entity.set_distance(distance);
    //         }

    //         slvs_entity.param
    //     };

    //     if let Some(param_vals) = entity_data.param_vals() {
    //         for (h, val) in zip(param_h, param_vals) {
    //             self.update_param(h, val)?;
    //         }
    //     }
    //     Ok(entity_data)
    // }

    pub fn update_constraint<C, F>(
        &mut self,
        constraint: &Constraint<C>,
        f: F,
    ) -> Result<C, &'static str>
    where
        C: AsConstraintData + 'static,
        F: FnOnce(&mut C),
    {
        let mut constraint_data = self.constraint_data(constraint)?;

        f(&mut constraint_data);
        self.validate_constraint_data(&constraint_data)?;

        let slvs_constraint = self.mut_slvs_constraint(constraint.as_handle()).unwrap();

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
        let ix = self.group_ix(group.as_handle())?;
        self.groups.list.remove(ix);
        Ok(group)
    }

    fn delete_param(&mut self, h: Slvs_hParam) -> Result<(), &'static str> {
        let ix = self.param_ix(h)?;
        self.params.list.remove(ix);

        Ok(())
    }

    pub fn delete_entity<E, T>(&mut self, entity: Entity<E>) -> Result<E, &'static str>
    where
        E: FromSlvsEntity<T>,
        T: AsTarget,
    {
        let entity_data = self.entity_data(&entity)?;
        let ix = self.entity_ix(entity.as_handle())?;
        let deleted_entity = self.entities.list.remove(ix);

        for param_h in deleted_entity.param {
            self.delete_param(param_h)?
        }

        Ok(entity_data)
    }

    pub fn delete_constraint<C: AsConstraintData + 'static>(
        &mut self,
        constraint: Constraint<C>,
    ) -> Result<C, &'static str> {
        let constraint_data = self.constraint_data(&constraint)?;

        let ix = self.constraint_ix(constraint.as_handle())?;
        self.constraints.list.remove(ix);

        Ok(constraint_data)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Solving the system
////////////////////////////////////////////////////////////////////////////////

impl System {
    pub fn set_dragged(&mut self, entity: &Entity<impl AsEntityData>) {
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
                failed_constraints: failed_handles.into_iter().filter(|&c_h| c_h != 0).collect(),
            }),
            Err(_) => Ok(SolveOkay {
                dof: slvs_system.dof,
            }),
        }
    }
}

#[derive(Debug)]
pub struct SolveOkay {
    pub dof: i32,
}

#[derive(Debug)]
pub struct SolveFail {
    pub dof: i32,
    pub reason: FailReason,
    pub failed_constraints: Vec<Slvs_hConstraint>,
}

impl SolveFail {
    pub fn constraint_did_fail<C: AsConstraintData>(&self, constraint: &Constraint<C>) -> bool {
        self.failed_constraints.contains(&constraint.as_handle())
    }
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
        Ok(&mut self.entities.list[ix])
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
