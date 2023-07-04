use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{
        Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_RADIUS, SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE,
    },
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, Circle, EntityHandle},
    group::Group,
    System,
};

/// `arc_a` and `arc_b` have an equal radius.
///
/// The entities can be either an [`ArcOfCircle`][crate::entity::ArcOfCircle] or a
/// [`Circle`][crate::entity::Circle], in any combination.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EqualRadius {
    Arcs {
        group: Group,
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
    },
    ArcAndCircle {
        group: Group,
        arc: EntityHandle<ArcOfCircle>,
        circle: EntityHandle<Circle>,
    },
    Circles {
        group: Group,
        circle_a: EntityHandle<Circle>,
        circle_b: EntityHandle<Circle>,
    },
}

impl EqualRadius {
    pub fn new_arcs(
        group: Group,
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
    ) -> Self {
        Self::Arcs {
            group,
            arc_a,
            arc_b,
        }
    }

    pub fn new_arc_and_circle(
        group: Group,
        arc: EntityHandle<ArcOfCircle>,
        circle: EntityHandle<Circle>,
    ) -> Self {
        Self::ArcAndCircle { group, arc, circle }
    }

    pub fn new_circles(
        group: Group,
        circle_a: EntityHandle<Circle>,
        circle_b: EntityHandle<Circle>,
    ) -> Self {
        Self::Circles {
            group,
            circle_a,
            circle_b,
        }
    }
}

impl AsGroup for EqualRadius {
    fn group(&self) -> Slvs_hGroup {
        match self {
            EqualRadius::Arcs { group, .. }
            | EqualRadius::ArcAndCircle { group, .. }
            | EqualRadius::Circles { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for EqualRadius {
    fn slvs_type(&self) -> i32 {
        SLVS_C_EQUAL_RADIUS as _
    }
}

impl AsConstraintData for EqualRadius {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        match self {
            EqualRadius::Arcs { arc_a, arc_b, .. } => Some([arc_a.handle(), arc_b.handle(), 0, 0]),
            EqualRadius::ArcAndCircle { arc, circle, .. } => {
                Some([arc.handle(), circle.handle(), 0, 0])
            }
            EqualRadius::Circles {
                circle_a, circle_b, ..
            } => Some([circle_a.handle(), circle_b.handle(), 0, 0]),
        }
    }
}

impl FromSystem for EqualRadius {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQUAL_RADIUS == slvs_constraint.type_ as _ {
            let radiused_a = sys.slvs_entity(slvs_constraint.entityA)?;
            let radiused_b = sys.slvs_entity(slvs_constraint.entityB)?;

            match (radiused_a.type_ as _, radiused_b.type_ as _) {
                (SLVS_E_ARC_OF_CIRCLE, SLVS_E_ARC_OF_CIRCLE) => Ok(EqualRadius::Arcs {
                    group: Group(slvs_constraint.group),

                    arc_a: EntityHandle::new(radiused_a.h),
                    arc_b: EntityHandle::new(radiused_b.h),
                }),
                (SLVS_E_ARC_OF_CIRCLE, SLVS_E_CIRCLE) => Ok(EqualRadius::ArcAndCircle {
                    group: Group(slvs_constraint.group),

                    arc: EntityHandle::new(radiused_a.h),
                    circle: EntityHandle::new(radiused_b.h),
                }),
                (SLVS_E_CIRCLE, SLVS_E_ARC_OF_CIRCLE) => Ok(EqualRadius::ArcAndCircle {
                    group: Group(slvs_constraint.group),

                    arc: EntityHandle::new(radiused_b.h),
                    circle: EntityHandle::new(radiused_a.h),
                }),
                (SLVS_E_CIRCLE, SLVS_E_CIRCLE) => Ok(EqualRadius::Circles {
                    group: Group(slvs_constraint.group),
                    circle_a: EntityHandle::new(radiused_a.h),
                    circle_b: EntityHandle::new(radiused_b.h),
                }),
                _ => Err("Expected constraint to apply to arcs and circles."),
            }
        } else {
            Err("Expected constraint to have type SLVS_C_EQUAL_RADIUS.")
        }
    }
}

// pub struct EqualRadius<AA, AB>
// where
//     AA: AsArc,
//     AB: AsArc,
// {
//     pub group: Group,
//     pub arc_a: EntityHandle<AA>,
//     pub arc_b: EntityHandle<AB>,
// }

// impl<AA, AB> EqualRadius<AA, AB>
// where
//     AA: AsArc,
//     AB: AsArc,
// {
//     pub fn new(group: Group, arc_a: EntityHandle<AA>, arc_b: EntityHandle<AB>) -> Self {
//         Self {
//             group,
//             arc_a,
//             arc_b,
//         }
//     }
// }

// impl<AA, AB> AsGroup for EqualRadius<AA, AB>
// where
//     AA: AsArc,
//     AB: AsArc,
// {
//     fn group(&self) -> Slvs_hGroup {
//         self.group.handle()
//     }
// }

// impl<AA, AB> AsSlvsType for EqualRadius<AA, AB>
// where
//     AA: AsArc,
//     AB: AsArc,
// {
//     fn slvs_type(&self) -> i32 {
//         SLVS_C_EQUAL_RADIUS as _
//     }
// }

// impl<AA, AB> AsConstraintData for EqualRadius<AA, AB>
// where
//     AA: AsArc,
//     AB: AsArc,
// {
//     fn workplane(&self) -> Option<Slvs_hEntity> {
//         None
//     }

//     fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
//         Some([self.arc_a.handle(), self.arc_b.handle(), 0, 0])
//     }
// }

// impl<AA, AB> FromSystem for EqualRadius<AA, AB>
// where
//     AA: AsArc,
//     AB: AsArc,
// {
//     fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
//     where
//         Self: Sized,
//     {
//         let slvs_constraint = sys.slvs_constraint(element.handle())?;

//         if SLVS_C_EQUAL_RADIUS == slvs_constraint.type_ as _ {
//             Ok(Self {
//                 group: Group(slvs_constraint.group),
//                 arc_a: EntityHandle::new(slvs_constraint.entityA),
//                 arc_b: EntityHandle::new(slvs_constraint.entityB),
//             })
//         } else {
//             Err("Expected constraint to have type SLVS_C_EQUAL_RADIUS.")
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{
        constraint::EqualRadius,
        entity::{ArcOfCircle, Circle, Distance, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion},
        System,
    };

    #[test]
    fn equal_radius() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-48.0, 65.0, -49.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([39.0, -64.0, 70.0], [-35.0, 74.0, -92.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let center = sys
            .sketch(Point::new_on_workplane(g, workplane, [-47.0, -51.0]))
            .expect("point created");
        let start = sys
            .sketch(Point::new_on_workplane(g, workplane, [-48.0, 24.0]))
            .expect("point created");
        let end = sys
            .sketch(Point::new_on_workplane(g, workplane, [-69.0, -78.0]))
            .expect("point created");
        let arc = sys
            .sketch(ArcOfCircle::new(g, workplane, center, start, end))
            .expect("arc created");

        let circle_center = sys
            .sketch(Point::new_in_3d(g, [8.0, 53.0, 49.0]))
            .expect("point created");
        let circle_radius = sys
            .sketch(Distance::new(g, 93.0))
            .expect("distance created");
        let circle = sys
            .sketch(Circle::new(g, normal, circle_center, circle_radius))
            .expect("circle created");

        sys.constrain(EqualRadius::new_arc_and_circle(g, arc, circle))
            .expect("constraint adde");

        dbg!(sys.solve(&g));

        let arc_r = if let (
            Point::OnWorkplane { coords: center, .. },
            Point::OnWorkplane {
                coords: arc_start, ..
            },
        ) = (
            sys.entity_data(&center).expect("data found"),
            sys.entity_data(&start).expect("data found"),
        ) {
            distance(center, arc_start)
        } else {
            unreachable!()
        };

        let circle_r = sys.entity_data(&circle_radius).expect("data found").val;

        len_within_tolerance!(arc_r, circle_r);
    }
}
