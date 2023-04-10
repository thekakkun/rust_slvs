#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use constraint::{AsConstraint, Constraint};
use entity::{AsEntity, Entity};
use group::Group;

pub mod constraint;
pub mod entity;
pub mod group;
pub mod param;

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SolveResult {
    None = binding::SLVS_RESULT_OKAY as _,
    Inconsistent = binding::SLVS_RESULT_INCONSISTENT as _,
    DidntConverge = binding::SLVS_RESULT_DIDNT_CONVERGE as _,
    TooManyUnknowns = binding::SLVS_RESULT_TOO_MANY_UNKNOWNS as _,
}

impl TryFrom<i32> for SolveResult {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, &'static str> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Inconsistent),
            2 => Ok(Self::DidntConverge),
            3 => Ok(Self::TooManyUnknowns),
            _ => Err("Failure can only take values 0, 1, 2, or 3."),
        }
    }
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
    calculateFaileds: bool,
    pub failed: Vec<Constraint>,
    pub dof: i32,
    pub result: SolveResult,
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
            calculateFaileds: true,
            failed: Vec::new(),
            dof: 0,
            result: SolveResult::None,
        }
    }
}

// Solving system
impl System {
    pub fn set_dragged(&mut self, entity: Entity) {
        if let Some(slvs_entity) = self.get_slvs_entity(entity.into()) {
            self.dragged = slvs_entity.param;
        }
    }

    pub fn clear_dragged(&mut self) {
        self.dragged = [0; 4];
    }

    pub fn solve(&mut self, group: Group) {
        self.failed = Vec::new();
        let mut failed_handles: Vec<binding::Slvs_hConstraint> = vec![0; self.constraints.len()];

        let mut slvs_system = binding::Slvs_System {
            param: self.params.as_mut_ptr(),
            params: self.params.len() as _,
            entity: self.entities.as_mut_ptr(),
            entities: self.entities.len() as _,
            constraint: self.constraints.as_mut_ptr(),
            constraints: self.constraints.len() as _,
            dragged: self.dragged,
            calculateFaileds: self.calculateFaileds as _,
            failed: failed_handles.as_mut_ptr(),
            faileds: failed_handles.len() as _,
            dof: self.dof,
            result: self.result as _,
        };

        unsafe {
            binding::Slvs_Solve(&mut slvs_system, group.into());

            failed_handles = Vec::from_raw_parts(
                slvs_system.failed,
                slvs_system.faileds.try_into().unwrap(),
                slvs_system.faileds.try_into().unwrap(),
            )
        };

        failed_handles
            .into_iter()
            .for_each(|h| self.failed.push(Constraint(h)));
        self.dof = slvs_system.dof;
        self.result = slvs_system.result.try_into().unwrap();
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

    pub fn add_entity(
        &mut self,
        group: Group,
        entity: impl AsEntity,
    ) -> Result<Entity, &'static str> {
        let params: [u32; 4] = entity
            .param_vals()
            .iter()
            .map(|&param| {
                if let Some(val) = param {
                    self.add_param(group, val)
                } else {
                    0
                }
            })
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        let new_entity = binding::Slvs_Entity {
            h: self.next_entity_h,
            group: group.into(),
            type_: entity.type_() as _,
            wrkpl: entity.wrkpl().unwrap_or(0), // TODO: check that entity exists and is the correct type
            point: entity.point().map(|p| p.unwrap_or(0)), // TODO: ditto
            normal: entity.normal().unwrap_or(0), // TODO: ditto
            distance: entity.distance().unwrap_or(0), // TODO: ditto
            param: params,
        };
        self.next_entity_h += 1;

        self.entities.push(new_entity);
        Ok(Entity(new_entity.h))
    }

    pub fn add_constraint(
        &mut self,
        group: Group,
        constraint: impl AsConstraint,
    ) -> Result<Constraint, &'static str> {
        let [pt_a, pt_b] = constraint.pt();
        let [entity_a, entity_b, entity_c, entity_d] = constraint.entity();
        let [other, other_2] = constraint.other();

        let new_constraint = binding::Slvs_Constraint {
            h: self.next_constraint_h,
            group: group.into(),
            type_: constraint.type_() as _,
            wrkpl: constraint.wrkpl().unwrap_or(0), // TODO: check that entity exists and is the correct type
            valA: constraint.valA(),
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
        Ok(Constraint(new_constraint.h))
    }

    // Private as user has no reason to create bare param without also creating entity.
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

// Getting elements
impl System {}

// Modifying elements
impl System {}

// Deleting elements
impl System {}

// Internal methods for Slvs_Handles -> &Slvs_Elements
impl System {
    fn get_slvs_param(&self, h: binding::Slvs_hParam) -> Option<&binding::Slvs_Param> {
        self.params.iter().find(|&param| param.h == h)
    }

    fn get_slvs_entity(&self, h: binding::Slvs_hEntity) -> Option<&binding::Slvs_Entity> {
        self.entities.iter().find(|&entity| entity.h == h)
    }

    fn get_slvs_constraint(
        &self,
        h: binding::Slvs_hConstraint,
    ) -> Option<&binding::Slvs_Constraint> {
        self.constraints
            .iter()
            .find(|&constraint| constraint.h == h)
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
            PtPtDistance {
                val: target_dist,
                wrkpl: None,
                pt_a: p1,
                pt_b: p2,
            },
        )
        .expect("distance constraint added");

        sys.set_dragged(p2);
        sys.solve(g);
        sys.clear_dragged();

        sys.params
            .iter()
            .for_each(|param| println!("{}: {:.3}", param.h, param.val));

        let dist = ((sys.params[0].val - sys.params[3].val).powi(2)
            + (sys.params[1].val - sys.params[4].val).powi(2)
            + (sys.params[2].val - sys.params[5].val).powi(2))
        .sqrt();

        assert!((target_dist - dist).abs() < SOLVE_TOLERANCE);
    }
}
