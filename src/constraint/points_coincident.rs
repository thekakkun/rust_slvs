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
    struct PointsCoincident {
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        /// If provided, constraint applies when projected onto this workplane.
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

#[cfg(test)]
mod tests {
    use crate::{
        constraint::PointsCoincident,
        entity::{Normal, Point, Workplane},
        len_within_tolerance,
        utils::{convert_2d_to_3d, distance, make_quaternion},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-23.0, -54.0, -67.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([14.0, 25.0, -81.0], [51.0, 99.0, -61.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [68.0, 43.0, 7.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-63.0, 33.0]))
            .expect("point created");

        sys.constrain(PointsCoincident::new(g, point_a, point_b, None))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::In3d {
                coords: coords_a, ..
            },
            Point::OnWorkplane {
                coords: coords_b, ..
            },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
        ) {
            let coords_b = convert_2d_to_3d(coords_b, origin, quaternion);

            len_within_tolerance!(distance(coords_a, coords_b), 0.0);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-10.0, 83.0, -31.0]))
            .expect("point  created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [25.0, 58.0, -84.0]))
            .expect("point  created");

        sys.constrain(PointsCoincident::new(g, point_a, point_b, None))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
        ) = (
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
        ) {
            len_within_tolerance!(distance(coords_a, coords_b), 0.0);
        } else {
            unreachable!()
        }
    }
}
