use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC_LINE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_SYMMETRIC_LINE,
    /// the points `point_a` and `point_b` are symmetric about `line`.
    struct SymmetricLine {
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        line: EntityHandle<LineSegment>,
    }
);

impl AsConstraintData for SymmetricLine {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point_a.handle(), self.point_b.handle()])
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line.handle(), 0, 0, 0])
    }
}

impl FromSystem for SymmetricLine {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_SYMMETRIC_LINE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                workplane: EntityHandle::new(slvs_constraint.wrkpl),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                line: EntityHandle::new(slvs_constraint.entityA),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_SYMMETRIC_LINE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::SymmetricLine,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{angle_2d, distance, make_quaternion, project_on_line, project_on_plane},
        System,
    };

    #[test]
    fn symmetric_line() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-61.0, -98.0, -89.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-25.0, -41.0, -92.0], [-18.0, -66.0, 36.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-30.0, 32.0, 31.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-14.0, -96.0, -75.0]))
            .expect("point created");

        let line_start = sys
            .sketch(Point::new_in_3d(g, [-30.0, 32.0, 31.0]))
            .expect("point created");
        let line_end = sys
            .sketch(Point::new_in_3d(g, [86.0, -48.0, -72.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, line_start, line_end))
            .expect("line created");

        sys.constrain(SymmetricLine::new(g, workplane, point_a, point_b, line))
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
            Point::In3d {
                coords: coords_line_start,
                ..
            },
            Point::In3d {
                coords: coords_line_end,
                ..
            },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
            sys.entity_data(&line_start).expect("data found"),
            sys.entity_data(&line_end).expect("data found"),
        ) {
            let coords_a = project_on_plane(coords_a, origin, quaternion);
            let coords_b = project_on_plane(coords_b, origin, quaternion);
            let coords_line_start = project_on_plane(coords_line_start, origin, quaternion);
            let coords_line_end = project_on_plane(coords_line_end, origin, quaternion);

            angle_within_tolerance!(
                angle_2d([coords_a, coords_b], [coords_line_start, coords_line_end]) % 180.0,
                90_f64
            );
            len_within_tolerance!(
                distance(
                    coords_a,
                    project_on_line(coords_a, coords_line_start, coords_line_end)
                ),
                distance(
                    coords_b,
                    project_on_line(coords_b, coords_line_start, coords_line_end)
                )
            );
        } else {
            unreachable!()
        }
    }
}
