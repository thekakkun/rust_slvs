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
    /// Note that the solver will fail if the two lines are initially parallel to eachother.
    ///
    /// # Examples
    ///
    /// ```
    /// use slvs::{
    ///     constraint::Angle,
    ///     entity::{LineSegment, Normal, Point, Workplane},
    ///     system::SOLVE_TOLERANCE,
    ///     utils::{angle, make_quaternion, project_3d_to_2d},
    ///     System,
    /// };
    ///
    /// let mut sys = System::new();
    ///
    /// let workplane_g = sys.add_group();
    /// let origin = sys
    ///     .sketch(Point::new_in_3d(workplane_g, [1.0, 2.0, 3.0]))
    ///     .expect("Origin created");
    /// let normal = sys
    ///     .sketch(Normal::new_in_3d(
    ///         workplane_g,
    ///         make_quaternion([4.0, 5.0, 6.0], [7.0, 8.0, 9.0]),
    ///     ))
    ///     .expect("normal created");
    /// let workplane = sys
    ///     .sketch(Workplane::new(workplane_g, origin, normal))
    ///     .expect("Workplane created");
    ///
    /// let g = sys.add_group();
    ///
    /// // Create line_ab
    /// let point_a = sys
    ///     .sketch(Point::new_in_3d(g, [10.0, 11.0, 12.0]))
    ///     .expect("point in 3d created");
    /// let point_b = sys
    ///     .sketch(Point::new_in_3d(g, [13.0, 14.0, 15.0]))
    ///     .expect("point in 3d created");
    /// let line_ab = sys
    ///     .sketch(LineSegment::new(g, point_a, point_b))
    ///     .expect("line between two 3d points created");
    ///
    /// // Create line_cd
    /// let point_c = sys
    ///     .sketch(Point::new_in_3d(g, [16.0, 17.0, 18.0]))
    ///     .expect("point in 3d created");
    /// let point_d = sys
    ///     .sketch(Point::new_in_3d(g, [19.0, 20.0, 22.0]))
    ///     .expect("point in 3d created");
    /// let line_cd = sys
    ///     .sketch(LineSegment::new(g, point_c, point_d))
    ///     .expect("line between two 3d points created");
    ///
    /// // Constrain angle between line_ab and line_cd to be 30 degrees apart.
    /// let angle_constraint = sys
    ///     .constrain(Angle::new(g, line_ab, line_cd, 30.0, None, false))
    ///     .expect("constraint added");
    ///
    /// sys.solve(&g);
    ///
    /// if let (
    ///     Point::In3d {
    ///         coords: coords_a, ..
    ///     },
    ///     Point::In3d {
    ///         coords: coords_b, ..
    ///     },
    ///     Point::In3d {
    ///         coords: coords_c, ..
    ///     },
    ///     Point::In3d {
    ///         coords: coords_d, ..
    ///     },
    /// ) = (
    ///     sys.entity_data(&point_a).expect("data for point_a found"),
    ///     sys.entity_data(&point_b).expect("data for point_b found"),
    ///     sys.entity_data(&point_c).expect("data for point_c found"),
    ///     sys.entity_data(&point_d).expect("data for point_d found"),
    /// ) {
    ///     assert!(angle([coords_a, coords_b], [coords_c, coords_d]) - 30.0 < SOLVE_TOLERANCE);
    /// } else {
    ///     unreachable!()
    /// }
    ///
    /// // Update the angle constraint.
    /// // Now, angle between the lines are 45 degrees, when projected onto workplane.
    /// sys.update_constraint(&angle_constraint, |constraint| {
    ///     constraint.angle = 45.0;
    ///     constraint.workplane = Some(workplane);
    /// })
    /// .expect("Lines are now 45 degrees apart, when projected on workplane");
    /// sys.solve(&g);
    ///
    /// if let (
    ///     Point::In3d { coords: origin, .. },
    ///     Normal::In3d { w, x, y, z, .. },
    ///     Point::In3d {
    ///         coords: coords_a, ..
    ///     },
    ///     Point::In3d {
    ///         coords: coords_b, ..
    ///     },
    ///     Point::In3d {
    ///         coords: coords_c, ..
    ///     },
    ///     Point::In3d {
    ///         coords: coords_d, ..
    ///     },
    /// ) = (
    ///     sys.entity_data(&origin).expect("data for origin found"),
    ///     sys.entity_data(&normal).expect("data for normal found"),
    ///     sys.entity_data(&point_a).expect("data for point_a found"),
    ///     sys.entity_data(&point_b).expect("data for point_b found"),
    ///     sys.entity_data(&point_c).expect("data for point_c found"),
    ///     sys.entity_data(&point_d).expect("data for point_d found"),
    /// ) {
    ///     let normal = [w, x, y, z];
    ///     assert!(
    ///         angle(
    ///             [
    ///                 project_3d_to_2d(coords_a, origin, normal),
    ///                 project_3d_to_2d(coords_b, origin, normal)
    ///             ],
    ///             [
    ///                 project_3d_to_2d(coords_c, origin, normal),
    ///                 project_3d_to_2d(coords_d, origin, normal)
    ///             ],
    ///         ) - 45.0
    ///             < SOLVE_TOLERANCE
    ///     );
    /// } else {
    ///     unreachable!()
    /// }
    /// ```
    struct Angle {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        /// The angle between `line_a` and `line_b`, in degrees.
        angle: f64,
        /// If provided, constrains the angle between the lines projected onto this
        /// workplane.
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
