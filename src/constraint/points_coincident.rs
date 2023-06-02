use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_POINTS_COINCIDENT},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_POINTS_COINCIDENT,
    /// `point_a` and `point_b` are coincident (i.e., exactly on top of each other).
    ///
    /// If `workplane` is provided, the points are coincident when projected onto
    /// the workplane. This should be provided if either of the points are already
    /// on a workplane (otherwise, the system becomes consistently over-constrained).
    ///
    /// # Example
    /// ```
    /// # use slvs::{
    /// # constraint::PointsCoincident,
    /// # entity::{Normal, Point, Workplane},
    /// # make_quaternion, System,
    /// # };
    /// #
    /// # let mut sys = System::new();
    /// # let workplane_g = sys.add_group();
    /// #
    /// # let origin = sys
    /// #     .sketch(Point::new_in_3d(workplane_g, [0.0; 3]))
    /// #     .expect("Origin created");
    /// # let normal = sys
    /// #     .sketch(Normal::new_in_3d(
    /// #         workplane_g,
    /// #         make_quaternion([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    /// #     ))
    /// #     .expect("normal created");
    /// # let workplane = sys
    /// #     .sketch(Workplane::new(workplane_g, origin, normal))
    /// #     .expect("Workplane created");
    /// #
    /// let g = sys.add_group();
    /// let point_a = sys
    ///     .sketch(Point::new_on_workplane(g, workplane, [10.0, 20.0]))
    ///     .expect("point in 2d created");
    /// let point_b = sys
    ///     .sketch(Point::new_on_workplane(g, workplane, [30.0, 40.0]))
    ///     .expect("point in 2d created");
    /// sys.constrain(PointsCoincident::new(g, point_a, point_b, Some(workplane)))
    ///     .expect("point_a and point_b are coincident");
    ///
    /// sys.solve(&g);
    ///
    /// if let (
    ///     Point::OnWorkplane {
    ///         coords: coords_a, ..
    ///     },
    ///     Point::OnWorkplane {
    ///         coords: coords_b, ..
    ///     },
    /// ) = (
    ///     sys.entity_data(&point_a).expect("data for point_a found"),
    ///     sys.entity_data(&point_b).expect("data for point_b found"),
    /// ) {
    ///     assert_eq!(coords_a, coords_b);
    /// }
    /// ```
    struct PointsCoincident {
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        /// If provided, will be coincident when the points are projected onto this
        /// workplane. Should be provided if either of the points are on a workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for PointsCoincident {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point_a.handle(), self.point_b.handle()])
    }
}

impl FromSystem for PointsCoincident {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_POINTS_COINCIDENT == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_POINTS_COINCIDENT.")
        }
    }
}
