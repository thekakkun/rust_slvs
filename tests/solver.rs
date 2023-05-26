use slvs::{constraint::PtPtDistance, entity::Point, solver::FailReason, System};

#[test]
fn inconsistent_constraints() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p1 = sys
        .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
        .expect("p1 created");
    let p2 = sys
        .sketch(Point::new_in_3d(g, [20.0, 20.0, 20.0]))
        .expect("p2 created");

    // distance between p1 and p2 is 10
    let c1 = sys
        .constrain(PtPtDistance::new(g, p1, p2, 10.0, None))
        .expect("distance constraint added");
    // distance between p1 and p2 is 20
    let c2 = sys
        .constrain(PtPtDistance::new(g, p1, p2, 20.0, None))
        .expect("distance constraint added");

    let solve_result = sys.solve(&g);

    if let Err(fail_result) = solve_result {
        assert_eq!(fail_result.reason, FailReason::Inconsistent);
        println!("{:?}", fail_result.failed_constraints);
        assert!(fail_result.constraint_failed(c1));
        assert!(fail_result.constraint_failed(c2));
    }
}
