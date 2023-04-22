use slvs::{
    entity::{Coords, Point},
    In3d, System,
};

#[test]
fn add_entity() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p_x = 10.0;
    let p_y = 20.0;
    let p_z = 30.0;

    let p = sys
        .sketch_in_3d(&g, Point::<In3d>::new(p_x, p_y, p_z))
        .expect("point in 3d created");

    let p_data = sys.entity_data(&p).expect("point found");
    if let Coords::In3d { x, y, z } = p_data.coords {
        assert_eq!(x, p_x);
        assert_eq!(y, p_y);
        assert_eq!(z, p_z);
    }
}

#[test]
fn update_entity() {
    let mut sys = slvs::System::new();
    let g = sys.add_group();

    let p = sys
        .sketch_in_3d(&g, Point::<In3d>::new(0.0, 0.0, 0.0))
        .expect("point in 3d created");

    let updated_p_x = 10.0;
    let updated_p_y = 20.0;
    let updated_p_z = 30.0;

    let updated_p_data = sys
        .update_entity(&p, |mut entity| {
            entity.coords = Coords::In3d {
                x: updated_p_x,
                y: updated_p_y,
                z: updated_p_z,
            }
        })
        .expect("should get updated point data");

    if let Coords::In3d { x, y, z } = updated_p_data.coords {
        assert_eq!(x, updated_p_x);
        assert_eq!(y, updated_p_y);
        assert_eq!(z, updated_p_z);
    }
}
