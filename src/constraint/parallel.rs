use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PARALLEL},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_PARALLEL,
    /// Lines `line_a` and `line_b` are parallel.
    /// 
    /// Note that this constraint in 3d space (`workplane` is `None`) is currently broken.
    struct Parallel {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for Parallel {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line_a.handle(), self.line_b.handle(), 0, 0])
    }
}

impl FromSystem for Parallel {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PARALLEL == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PARALLEL.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::Parallel,
        entity::{LineSegment, Normal, Point, Workplane},
        utils::{angle_2d, angle_3d, make_quaternion, project_on_plane, rounded_mod},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-42.0, 26.0, 25.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([89.0, 79.0, -76.0], [-57.0, -86.0, -74.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-59.0, -70.0, 90.0]))
            .expect("point  created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-85.0, -94.0, 19.0]))
            .expect("point  created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line  created");

        let point_c = sys
            .sketch(Point::new_in_3d(g, [81.0, 49.0, 82.0]))
            .expect("point  created");
        let point_d = sys
            .sketch(Point::new_in_3d(g, [-37.0, 10.0, -91.0]))
            .expect("point  created");
        let line_cd = sys
            .sketch(LineSegment::new(g, point_c, point_d))
            .expect("line  created");

        sys.constrain(Parallel::new(g, line_ab, line_cd, Some(workplane)))
            .expect("constraint added");
        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Point::In3d {
                coords: coords_c, ..
            },
            Point::In3d {
                coords: coords_d, ..
            },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
            sys.entity_data(&point_c).expect("data found"),
            sys.entity_data(&point_d).expect("data found"),
        ) {
            let angle = angle_2d(
                [
                    project_on_plane(coords_a, origin, quaternion),
                    project_on_plane(coords_b, origin, quaternion),
                ],
                [
                    project_on_plane(coords_c, origin, quaternion),
                    project_on_plane(coords_d, origin, quaternion),
                ],
            );

            angle_within_tolerance!(rounded_mod(angle, 180.0), 0_f64);
        } else {
            unreachable!()
        }
    }

    #[test]
    #[ignore] // Crashes due to bug in original library.
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [30.0, 62.0, 23.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [57.0, -3.0, -33.0]))
            .expect("point created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line created");

        let point_c = sys
            .sketch(Point::new_in_3d(g, [44.0, -18.0, 88.0]))
            .expect("point created");
        let point_d = sys
            .sketch(Point::new_in_3d(g, [-46.0, -23.0, 41.0]))
            .expect("point created");
        let line_cd = sys
            .sketch(LineSegment::new(g, point_c, point_d))
            .expect("line created");

        sys.constrain(Parallel::new(g, line_ab, line_cd, None))
            .expect("constraint added");
        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Point::In3d {
                coords: coords_c, ..
            },
            Point::In3d {
                coords: coords_d, ..
            },
        ) = (
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
            sys.entity_data(&point_c).expect("data found"),
            sys.entity_data(&point_d).expect("data found"),
        ) {
            let angle = angle_3d([coords_a, coords_b], [coords_c, coords_d]);

            angle_within_tolerance!(rounded_mod(angle, 180.0), 0_f64);
        } else {
            unreachable!()
        }
    }
}
