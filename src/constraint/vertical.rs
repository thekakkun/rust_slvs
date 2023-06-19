use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_VERTICAL},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
    System,
};

/// Constrain to be vertical.
///
/// This constraint can work on two points or a single line segment.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Vertical {
    /// Two points are vertical.
    Points {
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    },
    /// Line segment is vertical.
    Line {
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: EntityHandle<LineSegment>,
    },
}

impl Vertical {
    pub fn from_points(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    ) -> Self {
        Vertical::Points {
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
        Vertical::Line {
            group,
            workplane,
            line,
        }
    }
}

impl AsGroup for Vertical {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Vertical::Points { group, .. } | Vertical::Line { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Vertical {
    fn slvs_type(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }
}

impl AsConstraintData for Vertical {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Vertical::Points { workplane, .. } | Vertical::Line { workplane, .. } => {
                Some(workplane.handle())
            }
        }
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        match self {
            Vertical::Points {
                point_a, point_b, ..
            } => Some([point_a.handle(), point_b.handle()]),
            Vertical::Line { .. } => None,
        }
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        match self {
            Vertical::Points { .. } => None,
            Vertical::Line { line, .. } => Some([line.handle(), 0, 0, 0]),
        }
    }
}

impl FromSystem for Vertical {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_VERTICAL == slvs_constraint.type_ as _ {
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
                Err("Vertical should have handle for line or two points.")
            }
        } else {
            Err("Expected constraint to have type SLVS_C_VERTICAL.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::Vertical,
        entity::{LineSegment, Normal, Point, Workplane},
        len_within_tolerance,
        utils::make_quaternion,
        System,
    };

    #[test]
    fn verticalpoints() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-97.0, 22.0, 39.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-35.0, 23.0, 27.0], [-4.0, 77.0, -43.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let point_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [74.0, -56.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [71.0, 3.0]))
            .expect("point created");

        sys.constrain(Vertical::from_points(g, workplane, point_a, point_b))
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
            len_within_tolerance!(coords_a[0], coords_b[0]);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn verticalline() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [74.0, 60.0, 33.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([29.0, 25.0, 69.0], [91.0, 15.0, 81.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let point_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-70.0, -86.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [85.0, -17.0]))
            .expect("point created");
        let line = sys
            .sketch(LineSegment::new(g, point_a, point_b))
            .expect("point created");

        sys.constrain(Vertical::from_line(g, workplane, line))
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
            len_within_tolerance!(coords_a[0], coords_b[0]);
        } else {
            unreachable!()
        }
    }
}
