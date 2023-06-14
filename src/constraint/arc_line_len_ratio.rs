use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_LEN_RATIO},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, EntityHandle, LineSegment},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ARC_LINE_LEN_RATIO,
    /// Constrain the `arc` to be `ratio` times longer than `line`.
    struct ArcLineLenRatio {
        arc: EntityHandle<ArcOfCircle>,
        line: EntityHandle<LineSegment>,
        ratio: f64,
    }
);

impl AsConstraintData for ArcLineLenRatio {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line.handle(), self.arc.handle(), 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

impl FromSystem for ArcLineLenRatio {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ARC_LINE_LEN_RATIO == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                arc: EntityHandle::new(slvs_constraint.entityB),
                line: EntityHandle::new(slvs_constraint.entityA),
                ratio: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ARC_LINE_LEN_RATIO.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::ArcLineLenRatio,
        entity::{ArcOfCircle, LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{arc_len, distance, make_quaternion},
        System,
    };

    #[test]
    fn arc_line_len_ratio() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [76.0, 28.0, 62.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-42.0, -25.0, -5.0], [84.0, 81.0, 70.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [4.0, 30.0, -8.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-41.0, 62.0, -45.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line created");

        let center = sys
            .sketch(Point::new_on_workplane(g, workplane, [-10.0, 12.0]))
            .expect("point created");
        let arc_start = sys
            .sketch(Point::new_on_workplane(g, workplane, [37.0, 54.0]))
            .expect("point created");
        let arc_end = sys
            .sketch(Point::new_on_workplane(g, workplane, [25.0, -59.0]))
            .expect("point created");
        let arc = sys
            .sketch(ArcOfCircle::new(g, workplane, center, arc_start, arc_end))
            .expect("arc created");

        let len_difference = 5.0;
        sys.constrain(ArcLineLenRatio::new(g, arc, line, len_difference))
            .expect("constraint added");

        dbg!(sys.solve(&g));
        dbg!(sys.solve(&g));

        let line_len = if let (
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
            distance(coords_a, coords_b)
        } else {
            unreachable!();
        };

        let arc_len = if let (
            Point::OnWorkplane { coords: center, .. },
            Point::OnWorkplane {
                coords: arc_start, ..
            },
            Point::OnWorkplane {
                coords: arc_end, ..
            },
        ) = (
            sys.entity_data(&center).expect("data found"),
            sys.entity_data(&arc_start).expect("data found"),
            sys.entity_data(&arc_end).expect("data found"),
        ) {
            arc_len(center, arc_start, arc_end)
        } else {
            unreachable!()
        };

        len_within_tolerance!(arc_len / line_len, len_difference);
    }
}
