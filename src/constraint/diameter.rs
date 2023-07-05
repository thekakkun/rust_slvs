use serde::{Deserialize, Serialize};

use super::{AsConstraintData, SomeConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_DIAMETER, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, Circle, EntityHandle},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Diameter {
    Arc {
        group: Group,
        arc: EntityHandle<ArcOfCircle>,
        diameter: f64,
    },
    Circle {
        group: Group,
        circle: EntityHandle<Circle>,
        diameter: f64,
    },
}

impl Diameter {
    pub fn new_arc(group: Group, arc: EntityHandle<ArcOfCircle>, diameter: f64) -> Self {
        Self::Arc {
            group,
            arc,
            diameter,
        }
    }

    pub fn new_circle(group: Group, circle: EntityHandle<Circle>, diameter: f64) -> Self {
        Self::Circle {
            group,
            circle,
            diameter,
        }
    }
}

impl AsGroup for Diameter {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Diameter::Arc { group, .. } | Diameter::Circle { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Diameter {
    fn slvs_type(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }
}

impl AsConstraintData for Diameter {
    fn to_some_handle(handle: u32) -> SomeConstraintHandle {
        SomeConstraintHandle::Diameter(handle)
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        match self {
            Diameter::Arc { arc, .. } => Some([arc.handle(), 0, 0, 0]),
            Diameter::Circle { circle, .. } => Some([circle.handle(), 0, 0, 0]),
        }
    }

    fn val(&self) -> Option<f64> {
        match self {
            Diameter::Arc { diameter, .. } | Diameter::Circle { diameter, .. } => Some(*diameter),
        }
    }
}

impl FromSystem for Diameter {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_DIAMETER == slvs_constraint.type_ as _ {
            let target_entity = sys.slvs_entity(slvs_constraint.entityA)?;

            match target_entity.type_ as _ {
                SLVS_E_ARC_OF_CIRCLE => Ok(Diameter::Arc {
                    group: Group(slvs_constraint.group),
                    arc: EntityHandle::new(target_entity.h),
                    diameter: slvs_constraint.valA,
                }),
                SLVS_E_CIRCLE => Ok(Diameter::Circle {
                    group: Group(slvs_constraint.group),
                    circle: EntityHandle::new(target_entity.h),
                    diameter: slvs_constraint.valA,
                }),
                _ => Err("Expected constraint to apply to arc or circle."),
            }
        } else {
            Err("Expected constraint to have type SLVS_C_DIAMETER.")
        }
    }
}

// /// Constrain the diameter of [`ArcOfCircle`][crate::entity::ArcOfCircle] or [`Circle`][crate::entity::Circle]
// /// to equal `diameter`.
// #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
// pub struct Diameter<A: AsArc> {
//     pub group: Group,
//     pub arc: EntityHandle<A>,
//     pub diameter: f64,
// }

// impl<A: AsArc> Diameter<A> {
//     pub fn new(group: Group, arc: EntityHandle<A>, diameter: f64) -> Self {
//         Self {
//             group,
//             arc,
//             diameter,
//         }
//     }
// }

// impl<A: AsArc> AsGroup for Diameter<A> {
//     fn group(&self) -> Slvs_hGroup {
//         self.group.handle()
//     }
// }

// impl<A: AsArc> AsSlvsType for Diameter<A> {
//     fn slvs_type(&self) -> i32 {
//         SLVS_C_DIAMETER as _
//     }
// }

// impl<A: AsArc> AsConstraintData for Diameter<A> {
//     fn workplane(&self) -> Option<Slvs_hEntity> {
//         None
//     }

//     fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
//         Some([self.arc.handle(), 0, 0, 0])
//     }

//     fn val(&self) -> Option<f64> {
//         Some(self.diameter)
//     }
// }

// impl<A: AsArc> FromSystem for Diameter<A> {
//     fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
//     where
//         Self: Sized,
//     {
//         let slvs_constraint = sys.slvs_constraint(element.handle())?;

//         if SLVS_C_DIAMETER == slvs_constraint.type_ as _ {
//             Ok(Self {
//                 group: Group(slvs_constraint.group),
//                 arc: EntityHandle::new(slvs_constraint.entityA),
//                 diameter: slvs_constraint.valA,
//             })
//         } else {
//             Err("Expected constraint to have type SLVS_C_DIAMETER.")
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{
        constraint::Diameter,
        entity::{ArcOfCircle, Circle, Distance, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion},
        System,
    };

    #[test]
    fn arc_diameter() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [23.0, 12.0, -14.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([97.0, 17.0, 55.0], [57.0, -86.0, 95.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let center = sys
            .sketch(Point::new_on_workplane(g, workplane, [90.0, -5.0]))
            .expect("point created");
        let start = sys
            .sketch(Point::new_on_workplane(g, workplane, [-76.0, -13.0]))
            .expect("point created");
        let end = sys
            .sketch(Point::new_on_workplane(g, workplane, [-36.0, -75.0]))
            .expect("point created");
        let arc = sys
            .sketch(ArcOfCircle::new(g, workplane, center, start, end))
            .expect("arc created");

        sys.constrain(Diameter::new_arc(g, arc, 5.0))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::OnWorkplane { coords: center, .. },
            Point::OnWorkplane { coords: start, .. },
            Point::OnWorkplane { coords: end, .. },
        ) = (
            sys.entity_data(&center).expect("data found"),
            sys.entity_data(&start).expect("data found"),
            sys.entity_data(&end).expect("data found"),
        ) {
            len_within_tolerance!(distance(center, start), distance(center, end));
            len_within_tolerance!(distance(center, start) * 2.0, 5.0);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn circle_diameter() {
        let mut sys = System::new();

        let g = sys.add_group();
        let normal = sys
            .sketch(Normal::new_in_3d(
                g,
                make_quaternion([78.0, 60.0, 93.0], [49.0, 6.0, 73.0]),
            ))
            .expect("normal created");
        let circle_center = sys
            .sketch(Point::new_in_3d(g, [16.0, -58.0, 19.0]))
            .expect("point created");
        let circle_radius = sys
            .sketch(Distance::new(g, 30.0))
            .expect("distance created");
        let circle = sys
            .sketch(Circle::new(g, normal, circle_center, circle_radius))
            .expect("circle created");

        sys.constrain(Diameter::new_circle(g, circle, 5.0))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let Ok(circle_radius) = sys.entity_data(&circle_radius) {
            len_within_tolerance!(circle_radius.val * 2.0, 5.0);
        } else {
            unreachable!()
        }
    }
}
