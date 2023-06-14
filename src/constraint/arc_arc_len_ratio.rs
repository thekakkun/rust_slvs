use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_ARC_LEN_RATIO},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, EntityHandle},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ARC_ARC_LEN_RATIO,
    /// The length of `arc_a` divided by the length of `arc_b` is equal to `ratio`.
    struct ArcArcLenRatio {
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
        ratio: f64,
    }
);

impl AsConstraintData for ArcArcLenRatio {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc_a.handle(), self.arc_b.handle(), 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

impl FromSystem for ArcArcLenRatio {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ARC_ARC_LEN_RATIO == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                arc_a: EntityHandle::new(slvs_constraint.entityA),
                arc_b: EntityHandle::new(slvs_constraint.entityB),
                ratio: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ARC_ARC_LEN_RATIO.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::ArcArcLenRatio,
        entity::{ArcOfCircle, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{arc_len, make_quaternion},
        System,
    };

    #[test]
    fn arc_arc_len_ratio() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-69.0, -71.0, -49.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-80.0, -72.0, 36.0], [56.0, -57.0, 4.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let center_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-35.0, -2.0]))
            .expect("point created");
        let arc_start_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-67.0, -78.0]))
            .expect("point created");
        let arc_end_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-16.0, 94.0]))
            .expect("point created");
        let arc_a = sys
            .sketch(ArcOfCircle::new(
                g,
                workplane,
                center_a,
                arc_start_a,
                arc_end_a,
            ))
            .expect("arc created");

        let center_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-6.0, -69.0]))
            .expect("point created");
        let arc_start_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-41.0, 99.0]))
            .expect("point created");
        let arc_end_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [47.0, -70.0]))
            .expect("point created");
        let arc_b = sys
            .sketch(ArcOfCircle::new(
                g,
                workplane,
                center_b,
                arc_start_b,
                arc_end_b,
            ))
            .expect("arc created");

        let ratio = 2.5;
        sys.constrain(ArcArcLenRatio::new(g, arc_a, arc_b, ratio))
            .expect("constraint added");

        dbg!(sys.solve(&g));
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
            sys.entity_data(&center_a).expect("data found"),
            sys.entity_data(&arc_start_a).expect("data found"),
            sys.entity_data(&arc_end_a).expect("data found"),
        ) {
            arc_len(center, arc_start, arc_end)
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
            sys.entity_data(&center_b).expect("data found"),
            sys.entity_data(&arc_start_b).expect("data found"),
            sys.entity_data(&arc_end_b).expect("data found"),
        ) {
            arc_len(center, arc_start, arc_end)
        } else {
            unreachable!()
        };

        len_within_tolerance!(arc_a_len / arc_b_len, ratio);
    }
}
