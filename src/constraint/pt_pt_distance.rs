use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_PT_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_PT_PT_DISTANCE,
    /// The distance between `point_a` and `point_b` is equal to `distance`.
    ///
    /// This is an unsigned distance, so `distance` must always be positive.
    struct PtPtDistance {
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        distance: f64,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for PtPtDistance {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point_a.handle(), self.point_b.handle()])
    }
}

impl FromSystem for PtPtDistance {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_PT_DISTANCE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                distance: slvs_constraint.valA,
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_PT_DISTANCE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::PtPtDistance,
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
            .sketch(Point::new_in_3d(workplane_g, [-48.0, 13.0, 90.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([53.0, -22.0, 19.0], [-75.0, 9.0, 3.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-27.0, 79.0, -9.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-90.0, -40.0]))
            .expect("point created");

        let dist = 82.0;
        sys.constrain(PtPtDistance::new(g, point_a, point_b, dist, None))
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

            len_within_tolerance!(distance(coords_a, coords_b), dist);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-76.0, 21.0, -59.0]))
            .expect("point  created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [32.0, 98.0, -49.0]))
            .expect("point  created");

        let dist = 47.0;
        sys.constrain(PtPtDistance::new(g, point_a, point_b, dist, None))
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
            len_within_tolerance!(distance(coords_a, coords_b), dist);
        } else {
            unreachable!()
        }
    }
}
