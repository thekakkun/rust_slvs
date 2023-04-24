use slvs::{
    constraint::PtPtDistance,
    entity::{Coords, LineSegment, Point},
    FailReason, In3d, System,
};

const SOLVE_TOLERANCE: f64 = 1e-8;

#[test]
fn solve_3d_demo() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p1 = sys
        .sketch(&g, Point::<In3d>::new(10.0, 10.0, 10.0))
        .expect("p1 created");
    let p2 = sys
        .sketch(&g, Point::<In3d>::new(20.0, 20.0, 20.0))
        .expect("p2 created");

    sys.sketch(&g, LineSegment::<In3d>::new(&p1, &p2))
        .expect("line segment created");

    let target_dist = 30.0;
    sys.constrain_in_3d(&g, PtPtDistance::new(p1, p2, target_dist))
        .expect("distance constraint added");

    sys.set_dragged(&p2);
    let solve_result = sys.solve(&g);
    sys.clear_dragged();

    if solve_result.is_ok() {
        let new_p1 = sys.entity_data(&p1).expect("p1 should exist");
        let new_p2 = sys.entity_data(&p2).expect("p2 should exist");

        if let (
            Coords::In3d {
                x: x1,
                y: y1,
                z: z1,
            },
            Coords::In3d {
                x: x2,
                y: y2,
                z: z2,
            },
        ) = (new_p1.coords, new_p2.coords)
        {
            let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt();
            assert!((target_dist - dist).abs() < SOLVE_TOLERANCE);
        }
    }
}

#[test]
fn inconsistent_constraints() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p1 = sys
        .sketch(&g, Point::<In3d>::new(10.0, 10.0, 10.0))
        .expect("p1 created");
    let p2 = sys
        .sketch(&g, Point::<In3d>::new(20.0, 20.0, 20.0))
        .expect("p2 created");

    // distance between p1 and p2 is 10
    let c1 = sys
        .constrain_in_3d(&g, PtPtDistance::new(p1, p2, 10.0))
        .expect("distance constraint added");
    // distance between p1 and p2 is 20
    let c2 = sys
        .constrain_in_3d(&g, PtPtDistance::new(p1, p2, 20.0))
        .expect("distance constraint added");

    let solve_result = sys.solve(&g);

    if let Err(fail_result) = solve_result {
        assert_eq!(fail_result.reason, FailReason::Inconsistent);
        assert!(fail_result.constraint_did_fail(&c1));
        assert!(fail_result.constraint_did_fail(&c2));
    }
}
