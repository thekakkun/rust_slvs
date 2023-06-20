use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CUBIC_LINE_TANGENT},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{Cubic, EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_CUBIC_LINE_TANGENT,
    /// The `cubic` is tangent to the `line`.
    /// 
    /// Note that this constraint in 3d space (`workplane` is `None`) is currently broken.
    struct CubicLineTangent {
        cubic: EntityHandle<Cubic>,
        line: EntityHandle<LineSegment>,
        /// If `true` line is tangent to the end of the cubic instead of the start.
        to_end: bool,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for CubicLineTangent {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.cubic.handle(), self.line.handle(), 0, 0])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_end, false]
    }
}

impl FromSystem for CubicLineTangent {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_CUBIC_LINE_TANGENT == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                cubic: EntityHandle::new(slvs_constraint.entityA),
                line: EntityHandle::new(slvs_constraint.entityB),
                to_end: slvs_constraint.other != 0,
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_CUBIC_LINE_TANGENT.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::CubicLineTangent,
        entity::{Cubic, LineSegment, Normal, Point, Workplane},
        utils::{angle_2d, angle_3d, make_quaternion, rounded_mod},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-64.0, -80.0, -94.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([82.0, 11.0, -47.0], [91.0, 77.0, -93.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let start_point = sys
            .sketch(Point::new_on_workplane(g, workplane, [-37.0, 59.0]))
            .expect("point created");
        let start_control = sys
            .sketch(Point::new_on_workplane(g, workplane, [-35.0, -74.0]))
            .expect("point created");
        let end_control = sys
            .sketch(Point::new_on_workplane(g, workplane, [-62.0, 45.0]))
            .expect("point created");
        let end_point = sys
            .sketch(Point::new_on_workplane(g, workplane, [-65.0, 74.0]))
            .expect("point created");
        let cubic = sys
            .sketch(Cubic::new(
                g,
                start_point,
                start_control,
                end_control,
                end_point,
            ))
            .expect("cubic created");

        let point_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [58.0, -85.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [20.0, -62.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line created");

        sys.constrain(CubicLineTangent::new(g, cubic, line, false, None))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::OnWorkplane {
                coords: coords_start,
                ..
            },
            Point::OnWorkplane {
                coords: coords_control,
                ..
            },
            Point::OnWorkplane {
                coords: coords_a, ..
            },
            Point::OnWorkplane {
                coords: coords_b, ..
            },
        ) = (
            sys.entity_data(&start_point).expect("data found"),
            sys.entity_data(&start_control).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
        ) {
            let angle = angle_2d([coords_start, coords_control], [coords_a, coords_b]);
            angle_within_tolerance!(rounded_mod(angle, 180.0), 0_f64);
        } else {
            unreachable!()
        }
    }

    #[test]
    #[ignore] // Crashes due to bug in original library.
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();

        let start_point = sys
            .sketch(Point::new_in_3d(g, [-87.0, 58.0, 93.0]))
            .expect("point created");
        let start_control = sys
            .sketch(Point::new_in_3d(g, [6.0, -99.0, 14.0]))
            .expect("point created");
        let end_control = sys
            .sketch(Point::new_in_3d(g, [-64.0, -93.0, -75.0]))
            .expect("point created");
        let end_point = sys
            .sketch(Point::new_in_3d(g, [-67.0, -63.0, -72.0]))
            .expect("point created");
        let cubic = sys
            .sketch(Cubic::new(
                g,
                start_point,
                start_control,
                end_control,
                end_point,
            ))
            .expect("cubic created");

        let point_a = sys
            .sketch(Point::new_in_3d(g, [18.0, -34.0, -27.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-10.0, 95.0, 69.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("line created");

        sys.constrain(CubicLineTangent::new(g, cubic, line, false, None))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: coords_start,
                ..
            },
            Point::In3d {
                coords: coords_control,
                ..
            },
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
        ) = (
            sys.entity_data(&start_point).expect("data found"),
            sys.entity_data(&start_control).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
        ) {
            let angle = angle_3d([coords_start, coords_control], [coords_a, coords_b]);
            angle_within_tolerance!(rounded_mod(angle, 180.0), 0_f64);
        } else {
            unreachable!()
        }
    }
}
