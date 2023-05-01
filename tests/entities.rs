use slvs::{entity::Point, target::In3d, System};

#[test]
fn add_entity() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p_x = 10.0;
    let p_y = 20.0;
    let p_z = 30.0;

    let p = sys
        .sketch(Point::<In3d>::new(g, p_x, p_y, p_z))
        .expect("point in 3d created");

    let p_data = sys.entity_data(&p).expect("point found");
    let In3d(x, y, z) = p_data.coords;
    assert_eq!(x, p_x);
    assert_eq!(y, p_y);
    assert_eq!(z, p_z);
}

#[test]
fn update_entity() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p = sys
        .sketch(Point::<In3d>::new(g, 0.0, 0.0, 0.0))
        .expect("point in 3d created");

    let updated_p_x = 10.0;
    let updated_p_y = 20.0;
    let updated_p_z = 30.0;

    let updated_p_data = sys
        .update_entity(&p, |mut entity| {
            entity.coords = In3d(updated_p_x, updated_p_y, updated_p_z)
        })
        .expect("should get updated point data");

    let In3d(x, y, z) = updated_p_data.coords;
    assert_eq!(x, updated_p_x);
    assert_eq!(y, updated_p_y);
    assert_eq!(z, updated_p_z);
}
