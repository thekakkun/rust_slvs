use slvs::{
    constraint::PtPtDistance,
    entity::Point,
    system::{FailReason, SolveResult},
    System,
};

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
    sys.constrain(PtPtDistance::new(g, p1, p2, 10.0, None))
        .expect("distance constraint added");
    // distance between p1 and p2 is 20
    sys.constrain(PtPtDistance::new(g, p1, p2, 20.0, None))
        .expect("distance constraint added");

    let solve_result = sys.solve(&g);

    if let SolveResult::Fail {
        reason,
        failed_constraints,
        ..
    } = solve_result
    {
        assert_eq!(reason, FailReason::Inconsistent);
        println!("{:?}", failed_constraints);
    }
}
