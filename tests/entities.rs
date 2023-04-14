use slvs::{self, entity::PointIn3d};

#[test]
fn add_entity() {
    let mut sys = slvs::System::new();
    let g = sys.add_group();

    let p_x = 10.0;
    let p_y = 20.0;
    let p_z = 30.0;

    let p = sys
        .add_entity(
            g,
            PointIn3d {
                x: p_x,
                y: p_y,
                z: p_z,
            },
        )
        .expect("point in 3d created");

    let p_data: PointIn3d = sys
        .get_entity_data(p)
        .expect("point found")
        .try_into()
        .expect("returned data is for PointIn3d");

    assert_eq!(p_data.x, p_x);
    assert_eq!(p_data.y, p_y);
    assert_eq!(p_data.z, p_z);
}

#[test]
fn update_entity() {
    let mut sys = slvs::System::new();
    let g = sys.add_group();

    let p = sys
        .add_entity(
            g,
            PointIn3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        )
        .expect("point in 3d created");

    let updated_p_x = 10.0;
    let updated_p_y = 20.0;
    let updated_p_z = 30.0;

    let updated_p_data = sys
        .update_entity(p, |mut entity| {
            entity.x = updated_p_x;
            entity.y = updated_p_y;
            entity.z = updated_p_z;

            entity
        })
        .expect("should get updated point data");

    assert_eq!(updated_p_data.x, updated_p_x);
    assert_eq!(updated_p_data.y, updated_p_y);
    assert_eq!(updated_p_data.z, updated_p_z);
}