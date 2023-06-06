use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_ARC_DIFFERENCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, EntityHandle},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ARC_ARC_DIFFERENCE,
    /// The lengths of `arc_a` and `arc_b` differ by `difference`
    struct ArcArcDifference {
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
        difference: f64,
    }
);

impl AsConstraintData for ArcArcDifference {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc_a.handle(), self.arc_b.handle(), 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}

impl FromSystem for ArcArcDifference {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ARC_ARC_DIFFERENCE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                arc_a: EntityHandle::new(slvs_constraint.entityA),
                arc_b: EntityHandle::new(slvs_constraint.entityB),
                difference: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ARC_ARC_DIFFERENCE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::ArcArcDifference,
        entity::{ArcOfCircle, Normal, Point, Workplane},
        system::SOLVE_TOLERANCE,
        utils::{arc_len, make_quaternion},
        System,
    };

    #[test]
    fn arc_arc_difference() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [0.0, 74.0, 15.0]))
            .expect("Origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-20.0, 94.0, -24.0], [59.0, -88.0, 35.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("Workplane created");

        let g = sys.add_group();

        // Create arc_a
        let center_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-5.0, 56.0]))
            .expect("point on workplane created");
        let arc_start_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-62.0, -57.0]))
            .expect("point on workplane created");
        let arc_end_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-9.0, 42.0]))
            .expect("point on workplane created");
        let arc_a = sys
            .sketch(ArcOfCircle::new(
                g,
                workplane,
                center_a,
                arc_start_a,
                arc_end_a,
            ))
            .expect("Arc created");

        // Create arc_b
        let center_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-50.0, -10.0]))
            .expect("point on workplane created");
        let arc_start_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-41.0, -27.0]))
            .expect("point on workplane created");
        let arc_end_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-12.0, 26.0]))
            .expect("point on workplane created");
        let arc_b = sys
            .sketch(ArcOfCircle::new(
                g,
                workplane,
                center_b,
                arc_start_b,
                arc_end_b,
            ))
            .expect("Arc created");

        // Constrain arc lengths to have a difference of 100 units.
        let difference = 50.0;
        sys.constrain(ArcArcDifference::new(g, arc_a, arc_b, difference))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        let arc_a_len = if let (
            Point::OnWorkplane { coords: center, .. },
            Point::OnWorkplane {
                coords: arc_start, ..
            },
            Point::OnWorkplane {
                coords: arc_end, ..
            },
        ) = (
            sys.entity_data(&center_a).expect("center_a data found"),
            sys.entity_data(&arc_start_a)
                .expect("arc_start_a data found"),
            sys.entity_data(&arc_end_a).expect("arc_end_a data found"),
        ) {
            println!("{:?} {:?} {:?}", center, arc_start, arc_end);
            dbg!(arc_len(center, arc_start, arc_end))
        } else {
            unreachable!()
        };
        let arc_b_len = if let (
            Point::OnWorkplane { coords: center, .. },
            Point::OnWorkplane {
                coords: arc_start, ..
            },
            Point::OnWorkplane {
                coords: arc_end, ..
            },
        ) = (
            sys.entity_data(&center_b).expect("center_b data found"),
            sys.entity_data(&arc_start_b)
                .expect("arc_start_b data found"),
            sys.entity_data(&arc_end_b).expect("arc_end_b data found"),
        ) {
            println!("{:?} {:?} {:?}", center, arc_start, arc_end);
            dbg!(arc_len(center, arc_start, arc_end))
        } else {
            unreachable!()
        };

        assert!(((arc_a_len - arc_b_len).abs() - difference).abs() < SOLVE_TOLERANCE);
    }
}
