use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ANGLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ANGLE,
    /// Constrain the `angle` between `line_a` and `line_b`, in degrees.
    ///
    /// This constraint equation is written in the form
    ///
    /// ```text
    /// (A dot B)/(|A||B|) = cos(valA)
    /// ```
    ///
    /// where A and B are vectors in the directions of lines A and B. This equation
    /// does not specify the angle unambiguously; for example, note that `valA = +/- 90`
    /// degrees will produce the same equation.
    ///
    /// If `supplementary` is true, then the constraint is instead that
    ///
    /// ```text
    /// (A dot B)/(|A||B|) = -cos(valA)
    /// ```
    ///
    /// Note that the solver will fail if the two lines are initially parallel to each other.
    struct Angle {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        /// The angle between `line_a` and `line_b`, in degrees.
        angle: f64,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
        /// If `true`, sets the supplementary angle.
        supplementary: bool,
    }
);

impl AsConstraintData for Angle {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line_a.handle(), self.line_b.handle(), 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.angle)
    }

    fn others(&self) -> [bool; 2] {
        [self.supplementary, false]
    }
}

impl FromSystem for Angle {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ANGLE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                angle: slvs_constraint.valA,
                supplementary: slvs_constraint.other != 0,
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ANGLE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::Angle,
        entity::{LineSegment, Normal, Point, Workplane},
        utils::{angle_2d, angle_3d, make_quaternion, project_on_plane},
        System,
    };

    #[test]
    fn angle_on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [0.0, 0.0, 0.0]))
            .expect("Origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("Workplane created");

        let g = sys.add_group();

        // Create line_ab
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-26.0, 75.0, 4.0]))
            .expect("point in 3d created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [68.0, 63.0, -77.0]))
            .expect("point in 3d created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line between two 3d points created");

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

        // Constrain angle between line_ab and line_cd to be 30 degrees apart.
        sys.constrain(Angle::new(
            g,
            line_ab,
            line_cd,
            45.0,
            Some(workplane),
            false,
        ))
        .expect("constraint added");

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
            sys.entity_data(&point_c).expect("data for point_c found"),
            sys.entity_data(&point_d).expect("data for point_d found"),
        ) {
            let normal = [w, x, y, z];
            let angle = angle_2d(
                [
                    project_on_plane(coords_a, origin, normal),
                    project_on_plane(coords_b, origin, normal),
                ],
                [
                    project_on_plane(coords_c, origin, normal),
                    project_on_plane(coords_d, origin, normal),
                ],
            );

            angle_within_tolerance!(angle, 45_f64);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn angle_in_3d() {
        let mut sys = System::new();
        let g = sys.add_group();

        // Create line_ab
        let point_a = sys
            .sketch(Point::new_in_3d(g, [30.0, 62.0, 23.0]))
            .expect("point in 3d created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [57.0, -3.0, -33.0]))
            .expect("point in 3d created");
        let line_ab = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line between two 3d points created");

        // Create line_cd
        let point_c = sys
            .sketch(Point::new_in_3d(g, [44.0, -18.0, 88.0]))
            .expect("point in 3d created");
        let point_d = sys
            .sketch(Point::new_in_3d(g, [-46.0, -23.0, 41.0]))
            .expect("point in 3d created");
        let line_cd = sys
            .sketch(LineSegment::new(g, point_c, point_d))
            .expect("line between two 3d points created");

        // Constrain angle between line_ab and line_cd to be 30 degrees apart.
        sys.constrain(Angle::new(g, line_ab, line_cd, 150.0, None, false))
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
            sys.entity_data(&point_a).expect("data for point_a found"),
            sys.entity_data(&point_b).expect("data for point_b found"),
            sys.entity_data(&point_c).expect("data for point_c found"),
            sys.entity_data(&point_d).expect("data for point_d found"),
        ) {
            let angle = angle_3d([coords_a, coords_b], [coords_c, coords_d]);

            angle_within_tolerance!(angle, 150_f64);
        } else {
            unreachable!()
        }
    }
}
