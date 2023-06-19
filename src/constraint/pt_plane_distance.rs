use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_PLANE_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_PT_PLANE_DISTANCE,
    /// the distance between `point` and `plane` is equal to `distance`.
    ///
    /// This is a signed distance; positive vs. negative `distance` corresponds to a
    /// point that is above vs. below the plane.
    struct PtPlaneDistance {
        point: EntityHandle<Point>,
        plane: EntityHandle<Workplane>,
        distance: f64,
    }
);
impl AsConstraintData for PtPlaneDistance {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.plane.handle(), 0, 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}

impl FromSystem for PtPlaneDistance {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_PT_PLANE_DISTANCE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                plane: EntityHandle::new(slvs_constraint.entityA),
                distance: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_PT_PLANE_DISTANCE.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::PtPlaneDistance,
        entity::{Normal, Point, Workplane},
        len_within_tolerance,
        utils::{convert_2d_to_3d, distance, make_quaternion, project_on_plane},
        System,
    };

    #[test]
    fn pt_plane_distance() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [8.0, -69.0, -85.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([97.0, 52.0, -87.0], [9.0, 52.0, 83.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let point = sys
            .sketch(Point::new_in_3d(g, [-26.0, 75.0, 4.0]))
            .expect("point created");

        let dist = 96.0;
        sys.constrain(PtPlaneDistance::new(g, point, workplane, dist))
            .expect("constraint added");
        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::In3d { coords, .. },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&point).expect("data found"),
        ) {
            let projected_point = project_on_plane(coords, origin, quaternion);
            let projected_point_in_3d = convert_2d_to_3d(projected_point, origin, quaternion);

            len_within_tolerance!(distance(projected_point_in_3d, coords), dist);
        } else {
            unreachable!()
        }
    }
}
