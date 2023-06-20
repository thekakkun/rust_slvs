use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_LINE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_PT_ON_LINE,
    /// `point` lies on `line`.
    ///
    /// Note that this constraint removes one degree of freedom when projected
    /// in to the plane, but two degrees of freedom in 3d.
    struct PtOnLine {
        point: EntityHandle<Point>,
        line: EntityHandle<LineSegment>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for PtOnLine {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line.handle(), 0, 0, 0])
    }
}

impl FromSystem for PtOnLine {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_ON_LINE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                line: EntityHandle::new(slvs_constraint.entityA),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_ON_LINE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::PtOnLine,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion, project_on_line, project_on_plane},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [4.0, 89.0, 83.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([69.0, -98.0, 48.0], [-25.0, -50.0, 57.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point = sys
            .sketch(Point::new_in_3d(g, [-14.0, 67.0, -68.0]))
            .expect("point created");

        let line_start = sys
            .sketch(Point::new_in_3d(g, [86.0, -54.0, -38.0]))
            .expect("point created");
        let line_end = sys
            .sketch(Point::new_in_3d(g, [32.0, -92.0, -64.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, line_start, line_end))
            .expect("line created");

        sys.constrain(PtOnLine::new(g, point, line, Some(workplane)))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::In3d {
                coords: point_coords,
                ..
            },
            Point::In3d {
                coords: line_start_coords,
                ..
            },
            Point::In3d {
                coords: line_end_coords,
                ..
            },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point).expect("data found"),
            sys.entity_data(&line_start).expect("data found"),
            sys.entity_data(&line_end).expect("data found"),
        ) {
            let proj_pt_coords = project_on_plane(point_coords, origin, quaternion);
            let proj_line_start = project_on_plane(line_start_coords, origin, quaternion);
            let proj_line_end = project_on_plane(line_end_coords, origin, quaternion);

            let point_on_line = project_on_line(proj_pt_coords, proj_line_start, proj_line_end);

            len_within_tolerance!(distance(proj_pt_coords, point_on_line), 0.0);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();
        let point = sys
            .sketch(Point::new_in_3d(g, [-70.0, 67.0, -28.0]))
            .expect("point created");

        let line_start = sys
            .sketch(Point::new_in_3d(g, [-45.0, 82.0, -29.0]))
            .expect("point created");
        let line_end = sys
            .sketch(Point::new_in_3d(g, [67.0, 43.0, -31.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, line_start, line_end))
            .expect("line created");

        sys.constrain(PtOnLine::new(g, point, line, None))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: point_coords,
                ..
            },
            Point::In3d {
                coords: line_start_coords,
                ..
            },
            Point::In3d {
                coords: line_end_coords,
                ..
            },
        ) = (
            sys.entity_data(&point).expect("data found"),
            sys.entity_data(&line_start).expect("data found"),
            sys.entity_data(&line_end).expect("data found"),
        ) {
            let point_on_line = project_on_line(point_coords, line_start_coords, line_end_coords);

            len_within_tolerance!(distance(point_coords, point_on_line), 0.0);
        } else {
            unreachable!()
        }
    }
}
