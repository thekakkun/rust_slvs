use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SYMMETRIC},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_SYMMETRIC,
    /// The points `point_a` and `point_b` are symmetric about `plane`.
    ///
    /// This means that they are on opposite sides of the plane and at equal distances
    /// from the plane, and that the line connecting `point_a` and `point_b` is normal
    /// to the plane.
    struct Symmetric {
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
        plane: EntityHandle<Workplane>,
        /// If provided, `point_a` and `point_b` will be constrained so that
        /// the line connecting the two are parallel the symmetry plane's normal.
        workplane: Option<EntityHandle<Workplane>>,
    }
);

impl AsConstraintData for Symmetric {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point_a.handle(), self.point_b.handle()])
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.plane.handle(), 0, 0, 0])
    }
}

impl FromSystem for Symmetric {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_SYMMETRIC == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point_a: EntityHandle::new(slvs_constraint.ptA),
                point_b: EntityHandle::new(slvs_constraint.ptB),
                plane: EntityHandle::new(slvs_constraint.entityA),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_SYMMETRIC.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::Symmetric,
        entity::{Normal, Point, Workplane},
        len_within_tolerance,
        utils::{
            angle_2d, convert_2d_to_3d, distance, make_quaternion, project_on_line,
            project_on_plane, quaternion_n,
        },
        System,
    };

    #[test]
    fn on_workplane() {
        let mut sys = System::new();

        let g = sys.add_group();

        let symmetry_plane_origin = sys
            .sketch(Point::new_in_3d(g, [-59.0, -82.0, 8.0]))
            .expect("origin created");
        let symmetry_plane_normal = sys
            .sketch(Normal::new_in_3d(
                g,
                make_quaternion([-8.0, 95.0, 59.0], [-70.0, -23.0, -43.0]),
            ))
            .expect("normal created");
        let symmetry_plane = sys
            .sketch(Workplane::new(
                g,
                symmetry_plane_origin,
                symmetry_plane_normal,
            ))
            .expect("workplane created");

        let point_a = sys
            .sketch(Point::new_in_3d(g, [-1.0, -68.0, -93.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [45.0, -52.0, -59.0]))
            .expect("point created");

        let projection_plane_origin = sys
            .sketch(Point::new_in_3d(g, [99.0, -83.0, 8.0]))
            .expect("origin created");
        let projection_plane_normal = sys
            .sketch(Normal::new_in_3d(
                g,
                make_quaternion([-50.0, 52.0, -48.0], [51.0, -41.0, -68.0]),
            ))
            .expect("normal created");
        let projection_plane = sys
            .sketch(Workplane::new(
                g,
                projection_plane_origin,
                projection_plane_normal,
            ))
            .expect("workplane created");

        sys.constrain(Symmetric::new(
            g,
            point_a,
            point_b,
            symmetry_plane,
            Some(projection_plane),
        ))
        .expect("constraint added");
        dbg!(sys.solve(&g));

        if let (
            Normal::In3d {
                quaternion: symmetry_plane_quaternion,
                ..
            },
            Point::In3d {
                coords: coords_a, ..
            },
            Point::In3d {
                coords: coords_b, ..
            },
            Point::In3d {
                coords: projection_plane_origin_coords,
                ..
            },
            Normal::In3d {
                quaternion: projection_plane_quaternion,
                ..
            },
        ) = (
            sys.entity_data(&symmetry_plane_normal).expect("data found"),
            sys.entity_data(&point_a).expect("data found"),
            sys.entity_data(&point_b).expect("data found"),
            sys.entity_data(&projection_plane_origin)
                .expect("data found"),
            sys.entity_data(&projection_plane_normal)
                .expect("data found"),
        ) {
            let proj_pt_a = project_on_plane(
                coords_a,
                projection_plane_origin_coords,
                projection_plane_quaternion,
            );
            let proj_pt_b = project_on_plane(
                coords_b,
                projection_plane_origin_coords,
                projection_plane_quaternion,
            );

            let symmetry_vec = project_on_plane(
                quaternion_n(symmetry_plane_quaternion),
                [0.0; 3],
                projection_plane_quaternion,
            );
            // symmetry line is rotated 90 degrees from normal of the symmetry plane.
            let symmetry_vec = [symmetry_vec[1], -symmetry_vec[0]];
            angle_within_tolerance!(
                angle_2d([proj_pt_a, proj_pt_b], [[0.0; 2], symmetry_vec]) % 180.0,
                90_f64
            );

            let pt_a_on_line = project_on_line(proj_pt_a, [0.0; 2], symmetry_vec);
            let pt_b_on_line = project_on_line(proj_pt_b, [0.0; 2], symmetry_vec);
            len_within_tolerance!(distance(pt_a_on_line, pt_b_on_line), 0.0);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn in_3d() {
        let mut sys = System::new();

        let g = sys.add_group();
        let point_a = sys
            .sketch(Point::new_in_3d(g, [-62.0, -57.0, -34.0]))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [22.0, 52.0, -60.0]))
            .expect("point created");

        let origin = sys
            .sketch(Point::new_in_3d(g, [28.0, -9.0, -5.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                g,
                make_quaternion([-20.0, -40.0, 50.0], [7.0, 80.0, 67.0]),
            ))
            .expect("normal created");
        let symmetry_plane = sys
            .sketch(Workplane::new(g, origin, normal))
            .expect("workplane created");

        sys.constrain(Symmetric::new(g, point_a, point_b, symmetry_plane, None))
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
            let proj_pt_a = project_on_plane(coords_a, origin, quaternion);
            let proj_pt_b = project_on_plane(coords_b, origin, quaternion);
            let proj_pt_a_in_3d = convert_2d_to_3d(proj_pt_a, origin, quaternion);
            let proj_pt_b_in_3d = convert_2d_to_3d(proj_pt_b, origin, quaternion);

            len_within_tolerance!(distance(proj_pt_a, proj_pt_b), 0.0);
            len_within_tolerance!(
                distance(coords_a, proj_pt_a_in_3d),
                distance(coords_b, proj_pt_b_in_3d)
            );
        } else {
            unreachable!()
        }
    }
}
