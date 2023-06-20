use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PERPENDICULAR},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_PERPENDICULAR,
    /// Lines `line_a` and `line_b` are perpendicular.
    struct Perpendicular {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for Perpendicular {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line_a.handle(), self.line_b.handle(), 0, 0])
    }
}

impl FromSystem for Perpendicular {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PERPENDICULAR == slvs_constraint.type_ as _ {
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
            Err("Expected constraint to have type SLVS_C_PERPENDICULAR.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::Perpendicular,
        entity::{LineSegment, Normal, Point, Workplane},
        utils::{angle_2d, angle_3d, make_quaternion, project_on_plane},
        System,
    };

    #[test]
    fn parallel_on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-3.0, -57.0, -81.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-92.0, 46.0, 48.0], [62.0, 66.0, -78.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [79.0, -41.0, -24.0]))
            .expect("point  created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [30.0, -82.0, 1.0]))
            .expect("point  created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line  created");

        let point_c = sys
            .sketch(Point::new_in_3d(g, [67.0, -53.0, -37.0]))
            .expect("point  created");
        let point_d = sys
            .sketch(Point::new_in_3d(g, [39.0, 46.0, 72.0]))
            .expect("point  created");
        let line_cd = sys
            .sketch(LineSegment::new(g, point_c, point_d))
            .expect("line  created");

        sys.constrain(Perpendicular::new(g, line_ab, line_cd, Some(workplane)))
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

            angle_within_tolerance!(angle % 180.0, 90_f64);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn parallel_in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [92.0, 75.0, 13.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-87.0, -97.0, 3.0]))
            .expect("point created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line created");

        let point_c = sys
            .sketch(Point::new_in_3d(g, [-28.0, 81.0, -31.0]))
            .expect("point created");
        let point_d = sys
            .sketch(Point::new_in_3d(g, [18.0, -26.0, -37.0]))
            .expect("point created");
        let line_cd = sys
            .sketch(LineSegment::new(g, point_c, point_d))
            .expect("line created");

        sys.constrain(Perpendicular::new(g, line_ab, line_cd, None))
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

            angle_within_tolerance!(angle % 180.0, 90_f64);
        } else {
            unreachable!()
        }
    }
}
