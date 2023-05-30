use slvs::{constraint::PtPtDistance, entity::Point, System};

#[test]
fn i_use_this_to_format_doctests() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p1 = sys
        .sketch(Point::new_in_3d(g, [0.0, 0.0, 0.0]))
        .expect("p1 created");
    let p2 = sys
        .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
        .expect("p2 created");
    sys.constrain(PtPtDistance::new(g, p1, p2, 100.0, None))
        .expect("p1 and p2 are 100 units apart");

    sys.set_dragged(p1)
        .expect("Try not to move p1 when solving");
    sys.solve(&g);
    sys.clear_dragged();

    println!(
        "{:#?}",
        sys.entity_data(&p2)
            .expect("p1 should still be close to (0, 0, 0)")
    );
}
