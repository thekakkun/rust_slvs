use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQ_PT_LN_DISTANCES},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_EQ_PT_LN_DISTANCES,
    /// the distance between `line_a` and `point_a` are equal to the distance between
    /// `line_b` and `point_b`.
    struct EqPtLnDistances {
        line_a: EntityHandle<LineSegment>,
        point_a: EntityHandle<Point>,
        line_b: EntityHandle<LineSegment>,
        point_b: EntityHandle<Point>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);
impl AsConstraintData for EqPtLnDistances {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line_a.handle(), self.line_b.handle(), 0, 0])
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point_a.handle(), self.point_b.handle()])
    }
}

impl FromSystem for EqPtLnDistances {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQ_PT_LN_DISTANCES == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_EQ_PT_LN_DISTANCES.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::EqPtLnDistances,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion, project_on_line, project_on_plane},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [61.0, -65.0, -60.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-63.0, -2.0, 34.0], [-22.0, 22.0, 59.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let point_a_start = sys
            .sketch(Point::new_in_3d(g, [10.0, -2.0, -60.0]))
            .expect("point created");
        let point_a_end = sys
            .sketch(Point::new_in_3d(g, [-68.0, 85.0, -2.0]))
            .expect("point created");
        let line_a = sys
            .sketch(LineSegment::new(g, point_a_start, point_a_end))
            .expect("line created");

        let point_a = sys
            .sketch(Point::new_in_3d(g, [8.0, -52.0, 44.0]))
            .expect("point created");

        let point_b_start = sys
            .sketch(Point::new_in_3d(g, [-79.0, -46.0, 95.0]))
            .expect("point created");
        let point_b_end = sys
            .sketch(Point::new_in_3d(g, [-7.0, -47.0, 80.0]))
            .expect("point created");
        let line_b = sys
            .sketch(LineSegment::new(g, point_b_start, point_b_end))
            .expect("line created");

        let point_b = sys
            .sketch(Point::new_in_3d(g, [-88.0, -60.0, -70.0]))
            .expect("point created");

        sys.constrain(EqPtLnDistances::new(
            g,
            line_a,
            point_a,
            line_b,
            point_b,
            Some(workplane),
        ))
        .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (Point::In3d { coords: origin, .. }, Normal::In3d { quaternion, .. }) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
        ) {
            let dist_a = if let (
                Point::In3d {
                    coords: line_start, ..
                },
                Point::In3d {
                    coords: line_end, ..
                },
                Point::In3d { coords: point, .. },
            ) = (
                sys.entity_data(&point_a_start).expect("data found"),
                sys.entity_data(&point_a_end).expect("data found"),
                sys.entity_data(&point_a).expect("data found"),
            ) {
                let line_start = project_on_plane(line_start, origin, quaternion);
                let line_end = project_on_plane(line_end, origin, quaternion);
                let point = project_on_plane(point, origin, quaternion);

                distance(point, project_on_line(point, line_start, line_end))
            } else {
                unreachable!()
            };

            let dist_b = if let (
                Point::In3d {
                    coords: line_start, ..
                },
                Point::In3d {
                    coords: line_end, ..
                },
                Point::In3d { coords: point, .. },
            ) = (
                sys.entity_data(&point_b_start).expect("data found"),
                sys.entity_data(&point_b_end).expect("data found"),
                sys.entity_data(&point_b).expect("data found"),
            ) {
                let line_start = project_on_plane(line_start, origin, quaternion);
                let line_end = project_on_plane(line_end, origin, quaternion);
                let point = project_on_plane(point, origin, quaternion);

                distance(point, project_on_line(point, line_start, line_end))
            } else {
                unreachable!()
            };

            len_within_tolerance!(dist_a, dist_b);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();

        let point_a_start = sys
            .sketch(Point::new_in_3d(g, [-52.0, 65.0, 50.0]))
            .expect("point created");
        let point_a_end = sys
            .sketch(Point::new_in_3d(g, [-48.0, 90.0, -51.0]))
            .expect("point created");
        let line_a = sys
            .sketch(LineSegment::new(g, point_a_start, point_a_end))
            .expect("line created");

        let point_a = sys
            .sketch(Point::new_in_3d(g, [99.0, -93.0, -37.0]))
            .expect("point created");

        let point_b_start = sys
            .sketch(Point::new_in_3d(g, [93.0, -37.0, -60.0]))
            .expect("point created");
        let point_b_end = sys
            .sketch(Point::new_in_3d(g, [79.0, 60.0, 80.0]))
            .expect("point created");
        let line_b = sys
            .sketch(LineSegment::new(g, point_b_start, point_b_end))
            .expect("line created");

        let point_b = sys
            .sketch(Point::new_in_3d(g, [-35.0, 70.0, -86.0]))
            .expect("point created");

        sys.constrain(EqPtLnDistances::new(
            g, line_a, point_a, line_b, point_b, None,
        ))
        .expect("constraint added");

        dbg!(sys.solve(&g));

        let dist_a = if let (
            Point::In3d {
                coords: line_start, ..
            },
            Point::In3d {
                coords: line_end, ..
            },
            Point::In3d { coords: point, .. },
        ) = (
            sys.entity_data(&point_a_start).expect("data found"),
            sys.entity_data(&point_a_end).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
        ) {
            distance(point, project_on_line(point, line_start, line_end))
        } else {
            unreachable!()
        };

        let dist_b = if let (
            Point::In3d {
                coords: line_start, ..
            },
            Point::In3d {
                coords: line_end, ..
            },
            Point::In3d { coords: point, .. },
        ) = (
            sys.entity_data(&point_b_start).expect("data found"),
            sys.entity_data(&point_b_end).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
        ) {
            distance(point, project_on_line(point, line_start, line_end))
        } else {
            unreachable!()
        };

        len_within_tolerance!(dist_a, dist_b);
    }
}
