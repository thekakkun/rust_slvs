//! A variety of utility functions, for geometry calculations and conversions.

use euclid::{
    default::{Point2D, Point3D, Rotation3D, Size3D, Vector2D, Vector3D},
    Angle,
};

use std::{iter::zip, mem::MaybeUninit};

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
/// * `quaternion` - The normal of the plane, as a unit quaternion.
pub fn convert_2d_to_3d(point: [f64; 2], origin: [f64; 3], quaternion: [f64; 4]) -> [f64; 3] {
    let [w, i, j, k] = quaternion;
    let normal_quaternion = Rotation3D::unit_quaternion(i, j, k, w);
    let rotated_point = normal_quaternion.transform_point3d(Point2D::from(point).to_3d());

    (rotated_point + Size3D::from(origin)).into()
}

/// Project a point onto a line, in the same dimension space.
///
/// # Arguments
///
/// `point` - Coordinates of the point.
/// `line_start` - Coordinates of one end of the line.
/// `line_end` - Coordiantes of the other end of the line.
pub fn project_on_line<const N: usize>(
    point: [f64; N],
    line_start: [f64; N],
    line_end: [f64; N],
) -> [f64; N] {
    let line_direction: [f64; N] = zip(line_end, line_start)
        .map(|(e, s)| e - s)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let point_to_line_start: [f64; N] = zip(point, line_start)
        .map(|(e, s)| e - s)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let t = zip(line_direction, point_to_line_start)
        .map(|(a, b)| a * b)
        .sum::<f64>()
        / zip(line_direction, line_direction)
            .map(|(a, b)| a * b)
            .sum::<f64>();

    zip(line_start, line_direction)
        .map(|(s, d)| s + d * t)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

/// Project a point in 3d onto a plane
///
/// # Arguments
///
/// * `point` - Coordinates of the point.
/// * `origin` - Origin point of the plane to project onto.
/// * `quaternion` - The normal of the plane to project onto, as a unit quaternion.
pub fn project_on_plane(point: [f64; 3], origin: [f64; 3], quaternion: [f64; 4]) -> [f64; 2] {
    let [w, i, j, k] = quaternion;
    let normal_quaternion = Rotation3D::unit_quaternion(i, j, k, w);
    let rotated_point = normal_quaternion
        .inverse()
        .transform_point3d(Point3D::from(point) - Size3D::from(origin));
    rotated_point.to_2d().into()
}

/// Calculate the angle from `vec_a` to `vec_b`, in 2d space, going clockwise.
///
/// # Arguments
///
/// * `vec_a` - Start and end coordinates of the first vector.
/// * `vec_b` - Start and end coordinates of the second vector.
pub fn angle_2d(vec_a: [[f64; 2]; 2], vec_b: [[f64; 2]; 2]) -> f64 {
    let vec_a = Vector2D::from(vec_a[1]) - Vector2D::from(vec_a[0]);
    let vec_b = Vector2D::from(vec_b[1]) - Vector2D::from(vec_b[0]);

    // this uses a fast but inaccurate algorithm.
    // let angle = vec_a.angle_to(vec_b);
    let angle = Angle::radians(vec_b.y.atan2(vec_b.x) - vec_a.y.atan2(vec_a.x));

    angle.positive().to_degrees()
}

/// Calculate the shortest angle from `vec_a` to `vec_b`, in 3d space.
///
/// # Arguments
///
/// * `vec_a` - Start and end coordinates of the first vector.
/// * `vec_b` - Start and end coordinates of the second vector.
pub fn angle_3d(vec_a: [[f64; 3]; 2], vec_b: [[f64; 3]; 2]) -> f64 {
    let vec_a = Vector3D::from(vec_a[1]) - Vector3D::from(vec_a[0]);
    let vec_b = Vector3D::from(vec_b[1]) - Vector3D::from(vec_b[0]);

    // this uses a fast but inaccurate algorithm.
    // let angle = vec_a.angle_to(vec_b);
    let angle = Angle::radians(
        (vec_a.dot(vec_b) / (vec_a.length() * vec_b.length()))
            .clamp(-1.0, 1.0)
            .acos(),
    );

    angle.positive().to_degrees()
}

/// Calculate the length of the arc.
///
/// # Arguments
///
/// * `center` - The coordinates for the center of the arc.
/// * `arc_start` - The coordinates for the start of the arc, going counter-clockwise
/// from this point.
/// * `arc_end` - The coordinates for the end of the arc. Represents a full circle if coincident
/// with `start_point`.
///
/// # Panics
///
/// Function will panic if points do not define a circular arc, that is
///
/// ```text
/// distance(center, arc_start) == distance(center, arc_end)
/// ```
pub fn arc_len(center: [f64; 2], arc_start: [f64; 2], arc_end: [f64; 2]) -> f64 {
    let start_vec = Vector2D::from(arc_start) - Vector2D::from(center);
    let end_vec = Vector2D::from(arc_end) - Vector2D::from(center);

    crate::len_within_tolerance!(start_vec.length(), end_vec.length());

    let angle = Angle::radians(end_vec.y.atan2(end_vec.x) - start_vec.y.atan2(start_vec.x));
    angle.positive().radians * start_vec.length()
}

/// Returns the rounded modulo, where the remainder for `a/n` falls between `-n/2` and `n/2`.
///
/// # Arguments
///
/// * `a` - The dividend.
/// * `n` - The divisor.
pub fn rounded_mod(a: f64, n: f64) -> f64 {
    a - n * (a / n).round()
}

/// Asserts that two lengths are within [tolerance][crate::system::SOLVE_TOLERANCE].
///
/// If comparing angles, not lengths, use [`angle_within_tolerance`][crate::angle_within_tolerance].
#[macro_export]
macro_rules! len_within_tolerance {
    ($left:expr, $right:expr) => {
        assert!(
            // In reality, the difference should be much smaller than the tolerance.
            ($left - $right).abs() <= $crate::system::SOLVE_TOLERANCE * 1e-2,
            "assertion failed: `(left ≈ right)`
 left: `{}`,
right: `{}`",
            $left,
            $right
        )
    };
}

/// Asserts that two angles, in degrees are within [tolerance][crate::system::SOLVE_TOLERANCE].
///
/// Comparison is done between the cosine of the two angles.
///
/// If comparing lengths, not angles, use [`len_within_tolerance`].
#[macro_export]
macro_rules! angle_within_tolerance {
    ($left:expr, $right:expr) => {
        assert!(
            ($left.to_radians().cos() - $right.to_radians().cos()).abs()
                <= $crate::system::SOLVE_TOLERANCE,
            "assertion failed: `(left ≈ right)`
 left: `{}`,
right: `{}`",
            $left,
            $right
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, SQRT_2};

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

        len_within_tolerance!(
            (distance(
                coords_3d,
                [16.530612244897966, 46.734693877551024, 50.51020408163265]
            )),
            0.0
        );
    }

    #[test]
    fn test_3d_to_2d() {
        let coords_2d = project_on_plane(
            [34.89795918367347, 37.55102040816326, 56.63265306122449],
            [10.0, 20.0, 30.0],
            make_quaternion([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]),
        );

        len_within_tolerance!(
            distance(coords_2d, [7.142857142857142, 33.57142857142857]),
            0.0
        );
    }

    #[test]
    fn angles() {
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [1.0, 0.0]]),
            0_f64
        );
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [1.0, 1.0]]),
            45_f64
        );
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [0.0, 1.0]]),
            90_f64
        );
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [-1.0, 1.0]]),
            135_f64
        );
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [-1.0, 0.0]]),
            180_f64
        );
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [-1.0, -1.0]]),
            225_f64
        );
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [0.0, -1.0]]),
            270_f64
        );
        angle_within_tolerance!(
            angle_2d([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [1.0, -1.0]]),
            315_f64
        );
    }

    #[test]
    fn test_arc_len() {
        len_within_tolerance!(
            arc_len([0.0, 0.0], [1.0, 0.0], [SQRT_2 / 2.0, SQRT_2 / 2.0]),
            0.25 * PI
        );
        len_within_tolerance!(arc_len([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]), 0.5 * PI);
        len_within_tolerance!(
            arc_len([0.0, 0.0], [1.0, 0.0], [-SQRT_2 / 2.0, SQRT_2 / 2.0]),
            0.75 * PI
        );
        len_within_tolerance!(arc_len([0.0, 0.0], [1.0, 0.0], [-1.0, 0.0]), PI);
        len_within_tolerance!(
            arc_len([0.0, 0.0], [1.0, 0.0], [-SQRT_2 / 2.0, -SQRT_2 / 2.0]),
            1.25 * PI
        );
        len_within_tolerance!(arc_len([0.0, 0.0], [1.0, 0.0], [0.0, -1.0]), 1.5 * PI);
        len_within_tolerance!(
            arc_len([0.0, 0.0], [1.0, 0.0], [SQRT_2 / 2.0, -SQRT_2 / 2.0]),
            1.75 * PI
        );
    }
}
