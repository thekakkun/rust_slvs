use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_CIRCLE},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsArc, EntityHandle, Point},
    group::Group,
    System,
};

/// `point` lies on the right cyclinder obtained by extruding `arc` normal to its plane.
///
/// `arc` can be either a [`ArcOfCircle`][crate::entity::ArcOfCircle] or
/// [`Circle`][crate::entity::Circle]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PtOnCircle<A: AsArc> {
    pub group: Group,
    pub point: EntityHandle<Point>,
    pub arc: EntityHandle<A>,
}

impl<A: AsArc> PtOnCircle<A> {
    pub fn new(group: Group, point: EntityHandle<Point>, arc: EntityHandle<A>) -> Self {
        Self { group, point, arc }
    }
}

impl<A: AsArc> AsGroup for PtOnCircle<A> {
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<A: AsArc> AsSlvsType for PtOnCircle<A> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_CIRCLE as _
    }
}

impl<A: AsArc> AsConstraintData for PtOnCircle<A> {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc.handle(), 0, 0, 0])
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }
}

impl<A: AsArc> FromSystem for PtOnCircle<A> {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_ON_CIRCLE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                arc: EntityHandle::new(slvs_constraint.entityA),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_ON_CIRCLE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::{
        constraint::PtOnCircle,
        entity::{ArcOfCircle, Circle, Distance, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion, project_on_line, project_on_plane, quaternion_n},
        System,
    };

    #[test]
    fn pt_on_arc() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [25.0, -38.0, 97.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-96.0, -95.0, 12.0], [33.0, -85.0, 52.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point = sys
            .sketch(Point::new_in_3d(g, [-70.0, -77.0, -49.0]))
            .expect("point created");

        let center = sys
            .sketch(Point::new_on_workplane(g, workplane, [-15.0, 91.0]))
            .expect("point created");
        let start = sys
            .sketch(Point::new_on_workplane(g, workplane, [6.0, -24.0]))
            .expect("point created");
        let end = sys
            .sketch(Point::new_on_workplane(g, workplane, [79.0, 10.0]))
            .expect("point created");
        let arc = sys
            .sketch(ArcOfCircle::new(g, workplane, center, start, end))
            .expect("arc created");

        sys.constrain(PtOnCircle::new(g, point, arc))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::In3d {
                coords: point_coords,
                ..
            },
            Point::OnWorkplane {
                coords: center_coords,
                ..
            },
            Point::OnWorkplane {
                coords: start_coords,
                ..
            },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point).expect("data found"),
            sys.entity_data(&center).expect("data found"),
            sys.entity_data(&start).expect("data found"),
        ) {
            let proj_pt = project_on_plane(point_coords, origin, quaternion);

            len_within_tolerance!(
                distance(center_coords, start_coords),
                distance(center_coords, proj_pt)
            );
        } else {
            unreachable!()
        }
    }

    #[test]
    fn circle_diameter() {
        let mut sys = System::new();

        let g = sys.add_group();
        let point = sys
            .sketch(Point::new_in_3d(g, [-36.0, 32.0, 44.0]))
            .expect("point created");

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

        sys.constrain(PtOnCircle::new(g, point, circle))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: point_coords,
                ..
            },
            Normal::In3d { quaternion, .. },
            Point::In3d {
                coords: center_coords,
                ..
            },
            Distance { val: radius, .. },
        ) = (
            sys.entity_data(&point).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&circle_center).expect("data found"),
            sys.entity_data(&circle_radius).expect("data found"),
        ) {
            let normal_vec = quaternion_n(quaternion);
            let vec_end = zip(center_coords, normal_vec)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .expect("vec of len 3");
            let proj_point = project_on_line(point_coords, center_coords, vec_end);

            len_within_tolerance!(distance(point_coords, proj_point), radius);
        } else {
            unreachable!()
        }
    }
}
