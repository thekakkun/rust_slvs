use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC_HORIZ},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_SYMMETRIC_HORIZ,
    /// The points `point_a` and `point_b` are symmetric about the horizontal axis
    /// of the specified workplane.
    struct SymmetricHoriz {
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    }
);

impl AsConstraintData for SymmetricHoriz {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point_a.handle(), self.point_b.handle()])
    }
}

impl FromSystem for SymmetricHoriz {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_SYMMETRIC_HORIZ == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                workplane: EntityHandle::new(slvs_constraint.wrkpl),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                point_b: EntityHandle::new(slvs_constraint.ptB),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_SYMMETRIC_HORIZ.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::SymmetricHoriz,
        entity::{Normal, Point, Workplane},
        len_within_tolerance,
        utils::{make_quaternion, project_on_plane},
        System,
    };

    #[test]
    fn symmetric_horiz() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [34.0, 35.0, 41.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([75.0, -56.0, -40.0], [77.0, -78.0, -11.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [24.0, 37.0, 31.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-16.0, 26.0, -5.0]))
            .expect("point created");

        sys.constrain(SymmetricHoriz::new(g, workplane, point_a, point_b))
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
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
        ) {
            let coords_a = project_on_plane(coords_a, origin, quaternion);
            let coords_b = project_on_plane(coords_b, origin, quaternion);

            len_within_tolerance!(coords_a[0], -coords_b[0]);
            len_within_tolerance!(coords_a[1], coords_b[1]);
        } else {
            unreachable!()
        }
    }
}
