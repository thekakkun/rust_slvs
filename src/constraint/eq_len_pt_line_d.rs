use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_LEN_PT_LINE_D},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_EQ_LEN_PT_LINE_D,
    /// The length of `line_a` is equal to the distance from `point` to `line_b`
    struct EqLenPtLineD {
        line_a: EntityHandle<LineSegment>,
        point: EntityHandle<Point>,
        line_b: EntityHandle<LineSegment>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for EqLenPtLineD {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line_a.handle(), self.line_b.handle(), 0, 0])
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }
}

impl FromSystem for EqLenPtLineD {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQ_LEN_PT_LINE_D == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                point: EntityHandle::new(slvs_constraint.ptA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_EQ_LEN_PT_LINE_D.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EqLenPtLineD;
    use crate::{
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion, project_on_line, project_on_plane},
        System,
    };

    #[test]
    fn eq_len_pt_line_d_on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-48.0, -55.0, -27.0]))
            .expect("Origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([85.0, 33.0, -54.0], [-75.0, 3.0, 48.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("Workplane created");

        let g = sys.add_group();

        // Create line_ab
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-1.0, 53.0, -12.0]))
            .expect("point in 3d created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [66.0, 37.0, 10.0]))
            .expect("point in 3d created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line between two 3d points created");

        let point = sys
            .sketch(Point::new_in_3d(g, [36.0, 98.0, -51.0]))
            .expect("point created");

        // Create line_cd
        let point_c = sys
            .sketch(Point::new_in_3d(g, [60.0, 0.0, 15.0]))
            .expect("point in 3d created");
        let point_d = sys
            .sketch(Point::new_in_3d(g, [69.0, -69.0, 66.0]))
            .expect("point in 3d created");
        let line_cd = sys
            .sketch(LineSegment::new(g, point_c, point_d))
            .expect("line between two 3d points created");

        sys.constrain(EqLenPtLineD::new(
            g,
            line_ab,
            point,
            line_cd,
            Some(workplane),
        ))
        .expect("Constraint created");
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
            Point::In3d { coords: point, .. },
            Point::In3d {
                coords: coords_c, ..
            },
            Point::In3d {
                coords: coords_d, ..
            },
        ) = (
            sys.entity_data(&origin).expect("data for origin found"),
            sys.entity_data(&normal).expect("data for normal found"),
            sys.entity_data(&point_a).expect("data for point_a found"),
            sys.entity_data(&point_b).expect("data for point_b found"),
            sys.entity_data(&point).expect("data for point_b found"),
            sys.entity_data(&point_c).expect("data for point_c found"),
            sys.entity_data(&point_d).expect("data for point_d found"),
        ) {
            let normal = [w, x, y, z];
            let coords_a = project_on_plane(coords_a, origin, normal);
            let coords_b = project_on_plane(coords_b, origin, normal);
            let point = project_on_plane(point, origin, normal);
            let coords_c = project_on_plane(coords_c, origin, normal);
            let coords_d = project_on_plane(coords_d, origin, normal);

            let line_len = distance(coords_a, coords_b);
            let pt_line_dist = distance(point, project_on_line(point, coords_c, coords_d));

            len_within_tolerance!(line_len, pt_line_dist);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn eq_len_pt_line_d_in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();

        // Create line_ab
        let point_a = sys
            .sketch(Point::new_in_3d(g, [29.0, -74.0, -38.0]))
            .expect("point in 3d created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-31.0, -82.0, -90.0]))
            .expect("point in 3d created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line between two 3d points created");

        let point = sys
            .sketch(Point::new_in_3d(g, [-43.0, -59.0, 46.0]))
            .expect("point created");

        // Create line_cd
        let point_c = sys
            .sketch(Point::new_in_3d(g, [-58.0, 74.0, 97.0]))
            .expect("point in 3d created");
        let point_d = sys
            .sketch(Point::new_in_3d(g, [-65.0, -93.0, -29.0]))
            .expect("point in 3d created");
        let line_cd = sys
            .sketch(LineSegment::new(g, point_c, point_d))
            .expect("line between two 3d points created");

        sys.constrain(EqLenPtLineD::new(g, line_ab, point, line_cd, None))
            .expect("Constraint created");
        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Point::In3d { coords: point, .. },
            Point::In3d {
                coords: coords_c, ..
            },
            Point::In3d {
                coords: coords_d, ..
            },
        ) = (
            sys.entity_data(&point_a).expect("data for point_a found"),
            sys.entity_data(&point_b).expect("data for point_b found"),
            sys.entity_data(&point).expect("data for point_b found"),
            sys.entity_data(&point_c).expect("data for point_c found"),
            sys.entity_data(&point_d).expect("data for point_d found"),
        ) {
            let line_len = distance(coords_a, coords_b);
            let pt_line_dist = distance(point, project_on_line(point, coords_c, coords_d));

            len_within_tolerance!(line_len, pt_line_dist);
        } else {
            unreachable!()
        }
    }
}
