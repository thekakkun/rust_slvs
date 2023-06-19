use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_HORIZONTAL},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

/// Constrain to be horizontal.
///
/// This constraint can work on two points or a single line segment.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Horizontal {
    /// Two points are horizontal.
    Points {
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    },
    /// Line segment is horizontal.
    Line {
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: EntityHandle<LineSegment>,
    },
}

impl Horizontal {
    pub fn from_points(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    ) -> Self {
        Horizontal::Points {
            group,
            workplane,
            point_a,
            point_b,
        }
    }

    pub fn from_line(
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: EntityHandle<LineSegment>,
    ) -> Self {
        Horizontal::Line {
            group,
            workplane,
            line,
        }
    }
}

impl AsGroup for Horizontal {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Horizontal::Points { group, .. } | Horizontal::Line { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Horizontal {
    fn slvs_type(&self) -> i32 {
        SLVS_C_HORIZONTAL as _
    }
}

impl AsConstraintData for Horizontal {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Horizontal::Points { workplane, .. } | Horizontal::Line { workplane, .. } => {
                Some(workplane.handle())
            }
        }
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        match self {
            Horizontal::Points {
                point_a, point_b, ..
            } => Some([point_a.handle(), point_b.handle()]),
            Horizontal::Line { .. } => None,
        }
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        match self {
            Horizontal::Points { .. } => None,
            Horizontal::Line { line, .. } => Some([line.handle(), 0, 0, 0]),
        }
    }
}

impl FromSystem for Horizontal {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_HORIZONTAL == slvs_constraint.type_ as _ {
            if slvs_constraint.ptA != 0 && slvs_constraint.ptB != 0 {
                Ok(Self::Points {
                    group: Group(slvs_constraint.group),
                    workplane: EntityHandle::new(slvs_constraint.wrkpl),
                    point_a: EntityHandle::new(slvs_constraint.ptA),
                    point_b: EntityHandle::new(slvs_constraint.ptB),
                })
            } else if slvs_constraint.entityA != 0 {
                Ok(Self::Line {
                    group: Group(slvs_constraint.group),
                    workplane: EntityHandle::new(slvs_constraint.wrkpl),
                    line: EntityHandle::new(slvs_constraint.entityA),
                })
            } else {
                Err("Horizontal should have handle for line or two points.")
            }
        } else {
            Err("Expected constraint to have type SLVS_C_HORIZONTAL.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::Horizontal,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::make_quaternion,
        System,
    };

    #[test]
    fn horizontal_points() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-49.0, -2.0, 2.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-77.0, -26.0, 74.0], [0.0, 43.0, 78.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let point_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [82.0, 44.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [32.0, -2.0]))
            .expect("point created");

        sys.constrain(Horizontal::from_points(g, workplane, point_a, point_b))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::OnWorkplane {
                coords: coords_a, ..
            },
            Point::OnWorkplane {
                coords: coords_b, ..
            },
        ) = (
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
        ) {
            len_within_tolerance!(coords_a[1], coords_b[1]);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn horizontal_line() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-49.0, -2.0, 2.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-77.0, -26.0, 74.0], [0.0, 43.0, 78.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let point_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [82.0, 44.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [32.0, -2.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("point created");

        sys.constrain(Horizontal::from_line(g, workplane, line))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let (
            Point::OnWorkplane {
                coords: coords_a, ..
            },
            Point::OnWorkplane {
                coords: coords_b, ..
            },
        ) = (
            sys.entity_data(&point_a).expect("data for found"),
            sys.entity_data(&point_b).expect("data for found"),
        ) {
            len_within_tolerance!(coords_a[1], coords_b[1]);
        } else {
            unreachable!()
        }
    }
}
