//! A variety of utility functions, for geometry calculations and conversions.

use nalgebra::{Quaternion, UnitQuaternion, Vector3};
use std::mem::MaybeUninit;

use crate::bindings::{Slvs_MakeQuaternion, Slvs_QuaternionN, Slvs_QuaternionU, Slvs_QuaternionV};

/// Compute a unit quaternion from two basis vectors.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn make_quaternion(basis_vec_1: [f64; 3], basic_vec_2: [f64; 3]) -> [f64; 4] {
    let [ux, uy, uz] = basis_vec_1;
    let [vx, vy, vz] = basic_vec_2;

    let mut qw = MaybeUninit::<f64>::uninit();
    let mut qx = MaybeUninit::<f64>::uninit();
    let mut qy = MaybeUninit::<f64>::uninit();
    let mut qz = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_MakeQuaternion(
            ux,
            uy,
            uz,
            vx,
            vy,
            vz,
            qw.as_mut_ptr(),
            qx.as_mut_ptr(),
            qy.as_mut_ptr(),
            qz.as_mut_ptr(),
        );
        [
            qw.assume_init(),
            qx.assume_init(),
            qy.assume_init(),
            qz.assume_init(),
        ]
    }
}

/// Get the basis vector `U` from a quaternion.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn quaternion_u(quaternion: [f64; 4]) -> [f64; 3] {
    let [qw, qx, qy, qz] = quaternion;

    let mut x = MaybeUninit::<f64>::uninit();
    let mut y = MaybeUninit::<f64>::uninit();
    let mut z = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_QuaternionU(
            qw,
            qx,
            qy,
            qz,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            z.as_mut_ptr(),
        );

        [x.assume_init(), y.assume_init(), z.assume_init()]
    }
}

/// Get the basis vector `V` from a quaternion.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn quaternion_v(quaternion: [f64; 4]) -> [f64; 3] {
    let [qw, qx, qy, qz] = quaternion;

    let mut x = MaybeUninit::<f64>::uninit();
    let mut y = MaybeUninit::<f64>::uninit();
    let mut z = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_QuaternionV(
            qw,
            qx,
            qy,
            qz,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            z.as_mut_ptr(),
        );

        [x.assume_init(), y.assume_init(), z.assume_init()]
    }
}

/// Get the normal vector `N` from a quaternion.
///
/// See the documentation on [Normals][`crate::entity::Normal`] for more
/// information.
pub fn quaternion_n(quaternion: [f64; 4]) -> [f64; 3] {
    let [qw, qx, qy, qz] = quaternion;

    let mut x = MaybeUninit::<f64>::uninit();
    let mut y = MaybeUninit::<f64>::uninit();
    let mut z = MaybeUninit::<f64>::uninit();

    unsafe {
        Slvs_QuaternionN(
            qw,
            qx,
            qy,
            qz,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            z.as_mut_ptr(),
        );

        [x.assume_init(), y.assume_init(), z.assume_init()]
    }
}

/// Calculate the distance between cartesian coordinates.
///
/// # Arguments
///
/// * `coords_a`, `coords_b` - Coordinates to measure the distance between. Expected
/// to be of the same dimensionality.
pub fn distance<const N: usize>(coords_a: [f64; N], coords_b: [f64; N]) -> f64 {
    coords_a
        .iter()
        .zip(coords_b.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Convert 2d coordinates on a plane, into 3d coordinates.
///
/// # Arguments
///
/// * `point` - Coordinates of the point on the plane.
/// * `origin` - Origin point for the defintion of the plane.
/// * `normal` - The normal of the plane, as a unit quaternion.
pub fn convert_2d_to_3d(point: [f64; 2], origin: [f64; 3], normal: [f64; 4]) -> [f64; 3] {
    let [w, i, j, k] = normal;
    let normal_quaternion = UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k));

    let [u, v] = point;
    let rotated_point = normal_quaternion.transform_vector(&Vector3::new(u, v, 0.0));

    (Vector3::from(origin) + rotated_point).into()
}

/// Project a point in 3d onto a plane
///
/// # Arguments
///
/// * `point` - Coordinates of the point.
/// * `origin` - Origin point of the plane to project onto.
/// * `normal` - The normal of the plane to project onto, as a unit quaternion.
pub fn project_3d_to_2d(point: [f64; 3], origin: [f64; 3], normal: [f64; 4]) -> [f64; 2] {
    let [w, i, j, k] = normal;
    let normal_quaternion = UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k));

    let rotated_vector =
        normal_quaternion.inverse_transform_vector(&(Vector3::from(point) - Vector3::from(origin)));
    let [u, v, _]: [f64; 3] = rotated_vector.into();

    [u, v]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::SOLVE_TOLERANCE;

    #[test]
    fn test_distance() {
        assert_eq!(distance([3.0, 0.0], [0.0, 4.0]), 5.0);
        assert_eq!(distance([2.0, 0.0, 1.0], [0.0, 2.0, 0.0]), 3.0);
        assert_eq!(distance([1.0, 0.0, 1.0, 0.0], [0.0, 1.0, 0.0, 1.0]), 2.0);
    }

    #[test]
    fn test_2d_to_3d() {
        let coords_3d = convert_2d_to_3d(
            [7.142857142857142, 33.57142857142857],
            [10.0, 20.0, 30.0],
            make_quaternion([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]),
        );

        assert!(
            distance(
                coords_3d,
                [16.530612244897966, 46.734693877551024, 50.51020408163265]
            ) < SOLVE_TOLERANCE
        )
    }

    #[test]
    fn test_3d_to_2d() {
        let coords_2d = project_3d_to_2d(
            [34.89795918367347, 37.55102040816326, 56.63265306122449],
            [10.0, 20.0, 30.0],
            make_quaternion([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]),
        );

        assert!(distance(coords_2d, [7.142857142857142, 33.57142857142857]) < SOLVE_TOLERANCE)
    }
}
