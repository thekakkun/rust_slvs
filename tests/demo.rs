use slvs::{
    constraint::{Diameter, EqualRadius, LineVertical, PtLineDistance, PtPtDistance},
    entity::{ArcOfCircle, Circle, Coords, Distance, LineSegment, Normal, Point, Workplane},
    make_quaternion, In3d, OnWorkplane, System,
};

const SOLVE_TOLERANCE: f64 = 1e-8;

#[test]
fn solve_2d_demo() {
    let mut sys = System::new();

    println!("Define the workplane");
    let g1 = sys.add_group();
    let origin = sys
        .sketch(&g1, Point::<In3d>::new(0.0, 0.0, 0.0))
        .expect("origin point created");
    let normal = sys
        .sketch(
            &g1,
            Normal::<In3d>::new(make_quaternion([1.0, 0.0, 0.0], [0.0, 1.0, 0.0])),
        )
        .expect("normal created");
    let workplane = sys
        .sketch(&g1, Workplane::new(origin, normal))
        .expect("workplane created");

    let g2 = sys.add_group();

    println!("Draw a line segment");
    let p1 = sys
        .sketch(&g2, Point::<OnWorkplane>::new(workplane, 10.0, 20.0))
        .expect("point in 2d created");
    let p2 = sys
        .sketch(&g2, Point::<OnWorkplane>::new(workplane, 20.0, 10.0))
        .expect("point in 2d created");
    let line = sys
        .sketch(&g2, LineSegment::<OnWorkplane>::new(workplane, p1, p2))
        .expect("line segment created");

    println!("Draw an arc");
    let arc_center = sys
        .sketch(&g2, Point::<OnWorkplane>::new(workplane, 100.0, 120.0))
        .expect("point in 2d created");
    let arc_start = sys
        .sketch(&g2, Point::<OnWorkplane>::new(workplane, 120.0, 110.0))
        .expect("point in 2d created");
    let arc_end = sys
        .sketch(&g2, Point::<OnWorkplane>::new(workplane, 115.0, 115.0))
        .expect("point in 2d created");
    let arc = sys
        .sketch(
            &g2,
            ArcOfCircle::new(workplane, arc_center, arc_start, arc_end, normal),
        )
        .expect("arc created");

    println!("Draw a circle");
    let circle_center = sys
        .sketch(&g2, Point::<OnWorkplane>::new(workplane, 200.0, 200.0))
        .expect("point in 2d created");
    let circle_radius = sys
        .sketch(&g2, Distance::<OnWorkplane>::new(workplane, 30.0))
        .expect("distance created");
    let circle = sys
        .sketch(
            &g2,
            Circle::<OnWorkplane>::new(workplane, circle_center, circle_radius, normal),
        )
        .expect("circle created");

    let c1 = sys
        .constrain(&g2, PtPtDistance::new(p1, p2, 30.0, Some(workplane)))
        .expect("constrain line segment to 30.0 units");
    let c2 = sys
        .constrain(
            &g2,
            PtLineDistance::new(origin, line, 10.0, Some(workplane)),
        )
        .expect("distance from line to origin is 10.0");
    let c3 = sys
        .constrain(&g2, LineVertical::new(workplane, line))
        .expect("line segment is vertical");
    let c4 = sys
        .constrain(&g2, PtPtDistance::new(p1, origin, 15.0, Some(workplane)))
        .expect("distance from p1 to origin is 15.0 units");

    let c5 = sys
        .constrain(&g2, EqualRadius::new(arc, circle))
        .expect("arc and circle have equal radius");
    let c6 = sys
        .constrain(&g2, Diameter::new(arc, 17.0 * 2.0))
        .expect("arc has diameter of 17.0 units");

    let result = sys.solve(&g2);
    println!("{:?}", result);
}

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

    sys.sketch(&g, LineSegment::<In3d>::new(p1, p2))
        .expect("line segment created");

    let target_dist = 30.0;
    sys.constrain(&g, PtPtDistance::new(p1, p2, target_dist, None))
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
