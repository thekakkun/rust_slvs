use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_AT_MIDPOINT},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_AT_MIDPOINT,
    /// `point` lies at the the midpoint of `line`.
    struct AtMidpoint {
        point: EntityHandle<Point>,
        line: EntityHandle<LineSegment>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for AtMidpoint {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line.handle(), 0, 0, 0])
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }
}

impl FromSystem for AtMidpoint {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_AT_MIDPOINT == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                line: EntityHandle::new(slvs_constraint.entityA),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_AT_MIDPOINT.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::AtMidpoint,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{angle_2d, angle_3d, distance, make_quaternion, project_3d_to_2d},
        System,
    };

    #[test]
    fn at_midpoint_on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-64.0, -80.0, -94.0]))
            .expect("Origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([82.0, 11.0, -47.0], [91.0, 77.0, -93.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("Workplane created");

        let g = sys.add_group();

        // Create line_ab
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-66.0, -67.0, -43.0]))
            .expect("point in 3d created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-52.0, 73.0, 88.0]))
            .expect("point in 3d created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line between two 3d points created");

        // Create midpoint
        let point = sys
            .sketch(Point::new_in_3d(g, [-16.0, 38.0, -45.0]))
            .expect("point in 3d created");

        sys.constrain(AtMidpoint::new(g, point, line, Some(workplane)))
            .expect("point at midpoint of line");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { w, x, y, z, .. },
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Point::In3d { coords, .. },
        ) = (
            sys.entity_data(&origin).expect("data for origin found"),
            sys.entity_data(&normal).expect("data for normal found"),
            sys.entity_data(&point_a).expect("data for point_a found"),
            sys.entity_data(&point_b).expect("data for point_b found"),
            sys.entity_data(&point).expect("data for point_c found"),
        ) {
            let normal = [w, x, y, z];
            let point_a = project_3d_to_2d(coords_a, origin, normal);
            let point_b = project_3d_to_2d(coords_b, origin, normal);
            let point = project_3d_to_2d(coords, origin, normal);

            len_within_tolerance!(distance(point, point_a), distance(point, point_b));
            angle_within_tolerance!(angle_2d([point, point_a], [point, point_b]), 180_f64);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn at_midpoint_in_3d() {
        let mut sys = System::new();
        let g = sys.add_group();

        // Create line_ab
        let point_a = sys
            .sketch(Point::new_in_3d(g, [73.0, 36.0, 99.0]))
            .expect("point in 3d created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-52.0, -39.0, 33.0]))
            .expect("point in 3d created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line between two 3d points created");

        // Create midpoint
        let point = sys
            .sketch(Point::new_in_3d(g, [-5.0, -50.0, -76.0]))
            .expect("point in 3d created");

        sys.constrain(AtMidpoint::new(g, point, line, None))
            .expect("point at midpoint of line");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Point::In3d { coords, .. },
        ) = (
            sys.entity_data(&point_a).expect("data for point_a found"),
            sys.entity_data(&point_b).expect("data for point_b found"),
            sys.entity_data(&point).expect("data for point_c found"),
        ) {
            len_within_tolerance!(distance(coords, coords_a), distance(coords, coords_b));
            angle_within_tolerance!(angle_3d([coords, coords_a], [coords, coords_b]), 180_f64);
        } else {
            unreachable!()
        }
    }
}
