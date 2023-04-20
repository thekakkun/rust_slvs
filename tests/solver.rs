use slvs::{
    constraint::PtPtDistance,
    entity::{LineSegment, PointIn3d},
    FailReason, System,
};

const SOLVE_TOLERANCE: f64 = 1e-8;

#[test]
fn solve_3d_demo() {
    let mut sys = System::new();
    let g = sys.add_group();
    let p1 = sys
        .add_entity(&g, PointIn3d::new(10.0, 10.0, 10.0))
        .expect("p1 created");
    let p2 = sys
        .add_entity(&g, PointIn3d::new(20.0, 20.0, 20.0))
        .expect("p2 created");

    sys.add_entity(&g, LineSegment::new_in_3d(p1, p2))
        .expect("line segment created");

    let target_dist = 30.0;
    sys.add_constraint(
        &g,
        PtPtDistance::_3d {
            val: target_dist,
            point_a: p1,
            point_b: p2,
        },
    )
    .expect("distance constraint added");

    sys.set_dragged(&p2);
    let solve_result = sys.solve(&g);
    sys.clear_dragged();

    if solve_result.is_ok() {
        let new_p1 = sys.entity_data(&p1).expect("p1 should exist");
        let new_p2 = sys.entity_data(&p2).expect("p2 should exist");

        let dist = ((new_p1.x - new_p2.x).powi(2)
            + (new_p1.y - new_p2.y).powi(2)
            + (new_p1.z - new_p2.z).powi(2))
        .sqrt();

        assert!((target_dist - dist).abs() < SOLVE_TOLERANCE);
    }
}

#[test]
fn inconsistent_constraints() {
    let mut sys = System::new();
    let g = sys.add_group();
    let p1 = sys
        .add_entity(&g, PointIn3d::new(10.0, 10.0, 10.0))
        .expect("p1 created");

    let p2 = sys
        .add_entity(&g, PointIn3d::new(20.0, 20.0, 20.0))
        .expect("p2 created");

    // distance between p1 and p2 is 10
    let c1 = sys
        .add_constraint(
            &g,
            PtPtDistance::_3d {
                val: 10.0,
                point_a: p1,
                point_b: p2,
            },
        )
        .expect("distance constraint added");
    // distance between p1 and p2 is 20
    let c2 = sys
        .add_constraint(
            &g,
            PtPtDistance::_3d {
                val: 20.0,
                point_a: p1,
                point_b: p2,
            },
        )
        .expect("distance constraint added");

    let solve_result = sys.solve(&g);

    if let Err(fail_result) = solve_result {
        assert_eq!(fail_result.reason, FailReason::Inconsistent);
        assert!(fail_result.failed_constraints.contains(&c1.into()));
        assert!(fail_result.failed_constraints.contains(&c2.into()));
    }
}
