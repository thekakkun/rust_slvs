use slvs::{
    constraint::PtPtDistance,
    entity::{LineSegment, PointIn3d},
    System,
};

const SOLVE_TOLERANCE: f64 = 1e-8;

#[test]
fn solve_3d_demo() {
    let mut sys = System::new();
    let g = sys.add_group();
    let p1 = sys
        .add_entity(
            g,
            PointIn3d {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
        )
        .expect("p1 created");

    let p2 = sys
        .add_entity(
            g,
            PointIn3d {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
        )
        .expect("p2 created");

    sys.add_entity(
        g,
        LineSegment {
            point_a: p1,
            point_b: p2,
        },
    )
    .expect("line segment created");

    let target_dist = 30.0;
    sys.add_constraint(
        g,
        PtPtDistance::_3d {
            val: target_dist,
            point_a: p1,
            point_b: p2,
        },
    )
    .expect("distance constraint added");

    sys.set_dragged(p2);
    let solve_result = sys.solve(g);
    sys.clear_dragged();

    if solve_result.is_ok() {
        let new_p1 = sys.get_entity_data(p1).expect("p1 should exist");
        let new_p2 = sys.get_entity_data(p2).expect("p2 should exist");

        let dist = ((new_p1.x - new_p2.x).powi(2)
            + (new_p1.y - new_p2.y).powi(2)
            + (new_p1.z - new_p2.z).powi(2))
        .sqrt();

        assert!((target_dist - dist).abs() < SOLVE_TOLERANCE);
    }
}
