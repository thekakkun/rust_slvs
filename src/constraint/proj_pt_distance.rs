use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PROJ_PT_DISTANCE},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsProjectionTarget, EntityHandle, Point},
    group::Group,
    System,
};

/// The distance between `point_a` and `point_b`, when projected onto `line`
/// is equal to `distance`.
///
/// Here, `line` can be a [`LineSegment`][crate::entity::LineSegment] or
/// [`Normal`][crate::entity::Normal].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjPtDistance<L: AsProjectionTarget> {
    pub group: Group,
    pub point_a: EntityHandle<Point>,
    pub point_b: EntityHandle<Point>,
    pub line: EntityHandle<L>,
    pub distance: f64,
}

impl<L: AsProjectionTarget> ProjPtDistance<L> {
    pub fn new(
        group: Group,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        line: EntityHandle<L>,
        distance: f64,
    ) -> Self {
        Self {
            group,
            point_a,
            point_b,
            line,
            distance,
        }
    }
}
impl<L: AsProjectionTarget> AsGroup for ProjPtDistance<L> {
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<L: AsProjectionTarget> AsSlvsType for ProjPtDistance<L> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PROJ_PT_DISTANCE as _
    }
}

impl<L: AsProjectionTarget> AsConstraintData for ProjPtDistance<L> {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line.handle(), 0, 0, 0])
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point_a.handle(), self.point_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

impl<L: AsProjectionTarget> FromSystem for ProjPtDistance<L> {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PROJ_PT_DISTANCE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                line: EntityHandle::new(slvs_constraint.entityA),
                distance: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PROJ_PT_DISTANCE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::ProjPtDistance,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::{distance, make_quaternion, project_on_line, quaternion_n},
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-14.0, -1.0, -78.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([57.0, -25.0, 52.0], [-69.0, 24.0, -14.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [46.0, -65.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-96.0, -9.0]))
            .expect("point created");

        let line_start = sys
            .sketch(Point::new_on_workplane(g, workplane, [-82.0, 69.0]))
            .expect("point created");
        let line_end = sys
            .sketch(Point::new_on_workplane(g, workplane, [59.0, -42.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, line_start, line_end))
            .expect("line created");

        let dist = 92.0;
        sys.constrain(ProjPtDistance::new(g, point_a, point_b, line, dist))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::OnWorkplane {
                coords: coords_a, ..
            },
            Point::OnWorkplane {
                coords: coords_b, ..
            },
            Point::OnWorkplane {
                coords: line_start, ..
            },
            Point::OnWorkplane {
                coords: line_end, ..
            },
        ) = (
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
            sys.entity_data(&line_start).expect("data found"),
            sys.entity_data(&line_end).expect("data found"),
        ) {
            let proj_pt_a = project_on_line(coords_a, line_start, line_end);
            let proj_pt_b = project_on_line(coords_b, line_start, line_end);

            len_within_tolerance!(distance(proj_pt_a, proj_pt_b), dist);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();

        let point_a = sys
            .sketch(Point::new_in_3d(g, [44.0, -58.0, 95.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-78.0, -28.0, 76.0]))
            .expect("point created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                g,
                make_quaternion([13.0, 18.0, 76.0], [73.0, 9.0, -20.0]),
            ))
            .expect("normal created");

        let dist = 63.0;
        sys.constrain(ProjPtDistance::new(g, point_a, point_b, normal, dist))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Normal::In3d { quaternion, .. },
        ) = (
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
        ) {
            let normal_vec = quaternion_n(quaternion);
            let proj_pt_a = project_on_line(coords_a, [0.0; 3], normal_vec);
            let proj_pt_b = project_on_line(coords_b, [0.0; 3], normal_vec);

            len_within_tolerance!(distance(proj_pt_a, proj_pt_b), dist);
        } else {
            unreachable!();
        }
    }
}
