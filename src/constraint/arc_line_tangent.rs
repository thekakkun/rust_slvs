use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_TANGENT},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{ArcOfCircle, EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ARC_LINE_TANGENT,
    /// The `arc` is tangent to the `line`.
    ///
    /// If `to_end` is true, the arc is tangent at its end. Otherwise, the arc is tangent
    /// at its start.
    struct ArcLineTangent {
        workplane: EntityHandle<Workplane>,
        arc: EntityHandle<ArcOfCircle>,
        line: EntityHandle<LineSegment>,
        to_end: bool,
    }
);

impl AsConstraintData for ArcLineTangent {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.arc.handle(), self.line.handle(), 0, 0])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_end, false]
    }
}

impl FromSystem for ArcLineTangent {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ARC_LINE_TANGENT == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                workplane: EntityHandle::new(slvs_constraint.wrkpl),
                arc: EntityHandle::new(slvs_constraint.entityA),
                line: EntityHandle::new(slvs_constraint.entityB),
                to_end: slvs_constraint.other != 0,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ARC_LINE_TANGENT.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::ArcLineTangent,
        entity::{ArcOfCircle, LineSegment, Normal, Point, Workplane},
        utils::{angle_2d, make_quaternion, project_on_plane, rounded_mod},
        System,
    };

    #[test]
    fn arc_line_tangent() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-99.0, -90.0, 38.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([41.0, -59.0, -29.0], [-96.0, 42.0, 10.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [48.0, 29.0, 87.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-53.0, 69.0, 6.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line created");

        let center = sys
            .sketch(Point::new_on_workplane(g, workplane, [0.0, 46.0]))
            .expect("point created");
        let arc_start = sys
            .sketch(Point::new_on_workplane(g, workplane, [-48.0, 90.0]))
            .expect("point created");
        let arc_end = sys
            .sketch(Point::new_on_workplane(g, workplane, [98.0, 64.0]))
            .expect("point created");
        let arc = sys
            .sketch(ArcOfCircle::new(g, workplane, center, arc_start, arc_end))
            .expect("arc created");

        sys.constrain(ArcLineTangent::new(g, workplane, arc, line, false))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Point::OnWorkplane { coords: center, .. },
            Point::OnWorkplane { coords: start, .. },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
            sys.entity_data(&center).expect("data found"),
            sys.entity_data(&arc_start).expect("data found"),
        ) {
            let angle = angle_2d(
                [
                    project_on_plane(coords_a, origin, quaternion),
                    project_on_plane(coords_b, origin, quaternion),
                ],
                [center, start],
            );

            angle_within_tolerance!(rounded_mod(angle, 180.0), 90_f64);
        } else {
            unreachable!()
        }
    }
}
