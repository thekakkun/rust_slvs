use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_EQUAL_LINE_ARC_LEN},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, EntityHandle, LineSegment},
    group::Group,
    System,
};

define_element!(
    SLVS_C_EQUAL_LINE_ARC_LEN,
    /// The length of `line` is equal to the length of `arc`.
    struct EqualLineArcLen {
        line: EntityHandle<LineSegment>,
        arc: EntityHandle<ArcOfCircle>,
    }
);

impl AsConstraintData for EqualLineArcLen {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line.handle(), self.arc.handle(), 0, 0])
    }
}

impl FromSystem for EqualLineArcLen {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_EQUAL_LINE_ARC_LEN == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line: EntityHandle::new(slvs_constraint.entityA),
                arc: EntityHandle::new(slvs_constraint.entityB),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_EQUAL_LINE_ARC_LEN.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::EqualLineArcLen,
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
            .sketch(Point::new_in_3d(g, [-99.0, 83.0, 0.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-41.0, 62.0, -45.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line created");

        let center = sys
            .sketch(Point::new_on_workplane(g, workplane, [-64.0, 30.0]))
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

        sys.constrain(EqualLineArcLen::new(g, line, arc))
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
            sys.entity_data(&center).expect("center data found"),
            sys.entity_data(&arc_start).expect("arc_start data found"),
            sys.entity_data(&arc_end).expect("arc_end data found"),
        ) {
            arc_len(center, arc_start, arc_end)
        } else {
            unreachable!()
        };

        len_within_tolerance!(line_len, arc_len);
    }
}
