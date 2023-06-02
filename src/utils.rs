use nalgebra::{Quaternion, UnitQuaternion, Vector3};

pub fn distance<const N: usize>(coords_a: [f64; N], coords_b: [f64; N]) -> f64 {
    coords_a
        .iter()
        .zip(coords_b.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn convert_2d_to_3d(point: [f64; 2], origin: [f64; 3], normal: [f64; 4]) -> [f64; 3] {
    let [w, i, j, k] = normal;
    let normal_quaternion = UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k));

    let [u, v] = point;
    let rotated_point = normal_quaternion.transform_vector(&Vector3::new(u, v, 0.0));

    (Vector3::from(origin) + rotated_point).into()
}

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
    use crate::{make_quaternion, system::SOLVE_TOLERANCE};

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
