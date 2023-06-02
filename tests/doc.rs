use slvs::{
    constraint::PointsCoincident,
    entity::{Normal, Point, Workplane},
    utils::make_quaternion,
    System,
};

#[test]
fn i_use_this_to_format_doctests() {
    let mut sys = System::new();
    let g1 = sys.add_group();

    let origin = sys
        .sketch(Point::new_in_3d(g1, [10.0, 20.0, 30.0]))
        .expect("Origin created");
    let normal = sys
        .sketch(Normal::new_in_3d(
            g1,
            make_quaternion([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]),
        ))
        .expect("normal created");
    let workplane = sys
        .sketch(Workplane::new(g1, origin, normal))
        .expect("Workplane created");

    let g2 = sys.add_group();
    let point_a = sys
        .sketch(Point::new_on_workplane(g2, workplane, [10.0, 20.0]))
        .expect("point in 2d created");
    let point_b = sys
        .sketch(Point::new_in_3d(g2, [40.0, 50.0, 60.0]))
        .expect("point in 3d created");
    sys.constrain(PointsCoincident::new(g2, point_a, point_b, Some(workplane)))
        .expect("point_a and point_b are coincident");

    sys.solve(&g2);

    if let (
        Point::OnWorkplane {
            coords: coords_a, ..
        },
        Point::In3d {
            coords: coords_b, ..
        },
    ) = (
        sys.entity_data(&point_a).expect("data for point_a found"),
        sys.entity_data(&point_b).expect("data for point_b found"),
    ) {
        println!("{:?}", coords_a);
        println!("{:?}", coords_b);
    }
}
