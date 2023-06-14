use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_ANGLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_EQUAL_ANGLE,
    /// The angle between `line_a` and `line_b` is equal to the angle between `line_c`
    /// and `line_d`.
    ///
    /// If `supplementary` is true, the two angles add up to 180 degrees.
    struct EqualAngle {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        line_c: EntityHandle<LineSegment>,
        line_d: EntityHandle<LineSegment>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
        supplementary: bool,
    }
);

impl AsConstraintData for EqualAngle {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([
            self.line_a.handle(),
            self.line_b.handle(),
            self.line_c.handle(),
            self.line_d.handle(),
        ])
    }

    fn others(&self) -> [bool; 2] {
        [self.supplementary, false]
    }
}

impl FromSystem for EqualAngle {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQUAL_ANGLE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                line_c: EntityHandle::new(slvs_constraint.entityC),
                line_d: EntityHandle::new(slvs_constraint.entityD),
                supplementary: slvs_constraint.other != 0,
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_EQUAL_ANGLE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::EqualAngle,
        entity::{LineSegment, Normal, Point, Workplane},
        utils::{angle_2d, angle_3d, make_quaternion, project_on_plane},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-38.0, -53.0, -50.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([92.0, -96.0, -85.0], [-80.0, -27.0, 57.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let a_start = sys
            .sketch(Point::new_in_3d(g, [-26.0, 75.0, 4.0]))
            .expect("point created");
        let a_end = sys
            .sketch(Point::new_in_3d(g, [68.0, 63.0, -77.0]))
            .expect("point created");
        let line_a = sys
            .sketch(LineSegment::new(g, a_start, a_end))
            .expect("line created");

        let b_start = sys
            .sketch(Point::new_in_3d(g, [-26.0, 75.0, 4.0]))
            .expect("point created");
        let b_end = sys
            .sketch(Point::new_in_3d(g, [68.0, 63.0, -77.0]))
            .expect("point created");
        let line_b = sys
            .sketch(LineSegment::new(g, b_start, b_end))
            .expect("line created");

        let c_start = sys
            .sketch(Point::new_in_3d(g, [-26.0, 75.0, 4.0]))
            .expect("point created");
        let c_end = sys
            .sketch(Point::new_in_3d(g, [68.0, 63.0, -77.0]))
            .expect("point created");
        let line_c = sys
            .sketch(LineSegment::new(g, c_start, c_end))
            .expect("line created");

        let d_start = sys
            .sketch(Point::new_in_3d(g, [-26.0, 75.0, 4.0]))
            .expect("point created");
        let d_end = sys
            .sketch(Point::new_in_3d(g, [68.0, 63.0, -77.0]))
            .expect("point created");
        let line_d = sys
            .sketch(LineSegment::new(g, d_start, d_end))
            .expect("line created");

        sys.constrain(EqualAngle::new(
            g,
            line_a,
            line_b,
            line_c,
            line_d,
            Some(workplane),
            false,
        ))
        .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (Point::In3d { coords: origin, .. }, Normal::In3d { quaternion, .. }) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
        ) {
            let angle_ab = if let (
                Point::In3d {
                    coords: a_start, ..
                },
                Point::In3d { coords: a_end, .. },
                Point::In3d {
                    coords: b_start, ..
                },
                Point::In3d { coords: b_end, .. },
            ) = (
                sys.entity_data(&a_start).expect("data found"),
                sys.entity_data(&a_end).expect("data found"),
                sys.entity_data(&b_start).expect("data found"),
                sys.entity_data(&b_end).expect("data found"),
            ) {
                let a_start = project_on_plane(a_start, origin, quaternion);
                let a_end = project_on_plane(a_end, origin, quaternion);
                let b_start = project_on_plane(b_start, origin, quaternion);
                let b_end = project_on_plane(b_end, origin, quaternion);

                angle_2d([a_start, a_end], [b_start, b_end])
            } else {
                unreachable!()
            };

            let angle_cd = if let (
                Point::In3d {
                    coords: c_start, ..
                },
                Point::In3d { coords: c_end, .. },
                Point::In3d {
                    coords: d_start, ..
                },
                Point::In3d { coords: d_end, .. },
            ) = (
                sys.entity_data(&c_start).expect("data found"),
                sys.entity_data(&c_end).expect("data found"),
                sys.entity_data(&d_start).expect("data found"),
                sys.entity_data(&d_end).expect("data found"),
            ) {
                let c_start = project_on_plane(c_start, origin, quaternion);
                let c_end = project_on_plane(c_end, origin, quaternion);
                let d_start = project_on_plane(d_start, origin, quaternion);
                let d_end = project_on_plane(d_end, origin, quaternion);

                angle_2d([c_start, c_end], [d_start, d_end])
            } else {
                unreachable!()
            };

            angle_within_tolerance!(angle_ab, angle_cd);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();

        let a_start = sys
            .sketch(Point::new_in_3d(g, [24.0, -7.0, -36.0]))
            .expect("point created");
        let a_end = sys
            .sketch(Point::new_in_3d(g, [-95.0, 49.0, 19.0]))
            .expect("point created");
        let line_a = sys
            .sketch(LineSegment::new(g, a_start, a_end))
            .expect("line created");

        let b_start = sys
            .sketch(Point::new_in_3d(g, [-27.0, -19.0, 43.0]))
            .expect("point created");
        let b_end = sys
            .sketch(Point::new_in_3d(g, [20.0, -6.0, -5.0]))
            .expect("point created");
        let line_b = sys
            .sketch(LineSegment::new(g, b_start, b_end))
            .expect("line created");

        let c_start = sys
            .sketch(Point::new_in_3d(g, [26.0, -44.0, -24.0]))
            .expect("point created");
        let c_end = sys
            .sketch(Point::new_in_3d(g, [3.0, 34.0, 30.0]))
            .expect("point created");
        let line_c = sys
            .sketch(LineSegment::new(g, c_start, c_end))
            .expect("line created");

        let d_start = sys
            .sketch(Point::new_in_3d(g, [59.0, -43.0, 44.0]))
            .expect("point created");
        let d_end = sys
            .sketch(Point::new_in_3d(g, [-43.0, 52.0, -45.0]))
            .expect("point created");
        let line_d = sys
            .sketch(LineSegment::new(g, d_start, d_end))
            .expect("line created");

        sys.constrain(EqualAngle::new(
            g, line_a, line_b, line_c, line_d, None, true,
        ))
        .expect("constraint added");

        dbg!(sys.solve(&g));

        let angle_ab = if let (
            Point::In3d {
                coords: a_start, ..
            },
            Point::In3d { coords: a_end, .. },
            Point::In3d {
                coords: b_start, ..
            },
            Point::In3d { coords: b_end, .. },
        ) = (
            sys.entity_data(&a_start).expect("data found"),
            sys.entity_data(&a_end).expect("data found"),
            sys.entity_data(&b_start).expect("data found"),
            sys.entity_data(&b_end).expect("data found"),
        ) {
            angle_3d([a_start, a_end], [b_start, b_end])
        } else {
            unreachable!()
        };

        let angle_cd = if let (
            Point::In3d {
                coords: c_start, ..
            },
            Point::In3d { coords: c_end, .. },
            Point::In3d {
                coords: d_start, ..
            },
            Point::In3d { coords: d_end, .. },
        ) = (
            sys.entity_data(&c_start).expect("data found"),
            sys.entity_data(&c_end).expect("data found"),
            sys.entity_data(&d_start).expect("data found"),
            sys.entity_data(&d_end).expect("data found"),
        ) {
            angle_3d([c_start, c_end], [d_start, d_end])
        } else {
            unreachable!()
        };

        angle_within_tolerance!(angle_ab + angle_cd, 180_f64);
    }
}
