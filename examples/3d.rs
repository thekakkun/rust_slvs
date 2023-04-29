use slvs::{
    constraint::PtPtDistance,
    entity::{LineSegment, Point},
    target::In3d,
    System,
};

// An example of a constraint in 3d. We create a single group, with some
// entities and constraints.
fn main() {
    let mut sys = System::new();

    // This will contain a single group, which will arbitrarily number 1.
    let g = sys.add_group();

    // A point, initially at (x y z) = (10 10 10)
    let p1 = sys
        .sketch(&g, Point::<In3d>::new(10.0, 10.0, 10.0))
        .expect("p1 created");
    // and a second point at (20 20 20)
    let p2 = sys
        .sketch(&g, Point::<In3d>::new(20.0, 20.0, 20.0))
        .expect("p2 created");
    // and a line segment connecting them.
    sys.sketch(&g, LineSegment::<In3d>::new(p1, p2))
        .expect("line segment created");

    // The distance between the points should be 30.0 units.
    sys.constrain(&g, PtPtDistance::new(p1, p2, 30.0, None))
        .expect("distance constraint added");

    // Let's tell the solver to keep the second point as close to constant
    // as possible, instead moving the first point.
    sys.set_dragged(&p2);

    // Now that we have written our system, we solve.
    let result = sys.solve(&g);
    sys.clear_dragged();

    if let Ok(ok_result) = result {
        let In3d(x1, y1, z1) = sys.entity_data(&p1).expect("p1 should exist").coords;
        println!("okay; now at ({:.3} {:.3} {:.3})", x1, y1, z1);

        let In3d(x2, y2, z2) = sys.entity_data(&p2).expect("p2 should exist").coords;
        println!("             ({:.3} {:.3} {:.3})", x2, y2, z2);

        println!("{} DOF", ok_result.dof);
    } else {
        println!("solve failed");
    }
}
