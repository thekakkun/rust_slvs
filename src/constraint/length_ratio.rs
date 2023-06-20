use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_LENGTH_RATIO},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_LENGTH_RATIO,
    /// The length of `line_a` is `ratio` times longer than `line_b`.
    struct LengthRatio {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        ratio: f64,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for LengthRatio {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line_a.handle(), self.line_b.handle(), 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.ratio)
    }
}

impl FromSystem for LengthRatio {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_LENGTH_RATIO == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                ratio: slvs_constraint.valA,
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_LENGTH_RATIO.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::LengthRatio,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion, project_on_plane},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-20.0, 17.0, -59.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([89.0, -52.0, 94.0], [-4.0, -44.0, -15.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let a_start = sys
            .sketch(Point::new_on_workplane(g, workplane, [99.0, -6.0]))
            .expect("point created");
        let a_end = sys
            .sketch(Point::new_on_workplane(g, workplane, [70.0, 33.0]))
            .expect("point created");
        let line_a = sys
            .sketch(LineSegment::new(g, a_start, a_end))
            .expect("line created");

        let b_start = sys
            .sketch(Point::new_in_3d(g, [-58.0, 7.0, 56.0]))
            .expect("point created");
        let b_end = sys
            .sketch(Point::new_in_3d(g, [68.0, 63.0, -77.0]))
            .expect("point created");
        let line_b = sys
            .sketch(LineSegment::new(g, b_start, b_end))
            .expect("line created");

        let ratio = 2.5;
        sys.constrain(LengthRatio::new(g, line_a, line_b, ratio, Some(workplane)))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::OnWorkplane {
                coords: a_start, ..
            },
            Point::OnWorkplane { coords: a_end, .. },
            Point::In3d {
                coords: b_start, ..
            },
            Point::In3d { coords: b_end, .. },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&a_start).expect("data found"),
            sys.entity_data(&a_end).expect("data found"),
            sys.entity_data(&b_start).expect("data found"),
            sys.entity_data(&b_end).expect("data found"),
        ) {
            let b_start = project_on_plane(b_start, origin, quaternion);
            let b_end = project_on_plane(b_end, origin, quaternion);

            len_within_tolerance!(distance(a_start, a_end) / distance(b_start, b_end), ratio);
        } else {
            unreachable!()
        };
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-20.0, 17.0, -59.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([89.0, -52.0, 94.0], [-4.0, -44.0, -15.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let a_start = sys
            .sketch(Point::new_on_workplane(g, workplane, [99.0, -6.0]))
            .expect("point created");
        let a_end = sys
            .sketch(Point::new_on_workplane(g, workplane, [70.0, 33.0]))
            .expect("point created");
        let line_a = sys
            .sketch(LineSegment::new(g, a_start, a_end))
            .expect("line created");

        let b_start = sys
            .sketch(Point::new_in_3d(g, [-58.0, 7.0, 56.0]))
            .expect("point created");
        let b_end = sys
            .sketch(Point::new_in_3d(g, [68.0, 63.0, -77.0]))
            .expect("point created");
        let line_b = sys
            .sketch(LineSegment::new(g, b_start, b_end))
            .expect("line created");

        let ratio = 5.3;
        sys.constrain(LengthRatio::new(g, line_a, line_b, ratio, None))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::OnWorkplane {
                coords: a_start, ..
            },
            Point::OnWorkplane { coords: a_end, .. },
            Point::In3d {
                coords: b_start, ..
            },
            Point::In3d { coords: b_end, .. },
        ) = (
            sys.entity_data(&a_start).expect("data found"),
            sys.entity_data(&a_end).expect("data found"),
            sys.entity_data(&b_start).expect("data found"),
            sys.entity_data(&b_end).expect("data found"),
        ) {
            len_within_tolerance!(distance(a_start, a_end) / distance(b_start, b_end), ratio);
        } else {
            unreachable!()
        };
    }
}
