use slvs::{
    constraint::PtPtDistance,
    entity::{LineSegment, Point},
    system::SolveResult,
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
        .sketch(Point::new_in_3d(g, [10.0, 10.0, 10.0]))
        .expect("p1 created");
    // and a second point at (20 20 20)
    let p2 = sys
        .sketch(Point::new_in_3d(g, [20.0, 20.0, 20.0]))
        .expect("p2 created");
    // and a line segment connecting them.
    sys.sketch(LineSegment::new(g, p1, p2))
        .expect("line segment created");

    // The distance between the points should be 30.0 units.
    sys.constrain(PtPtDistance::new(g, p1, p2, 30.0, None))
        .expect("distance constraint added");

    // Let's tell the solver to keep the second point as close to constant
    // as possible, instead moving the first point.
    sys.set_dragged(p2).expect("p2 is locked in place");

    // Now that we have written our system, we solve.
    let result = sys.solve(&g);
    sys.clear_dragged();

    match result {
        SolveResult::Ok { dof } => {
            if let Point::In3d {
                coords: [x1, y1, z1],
                ..
            } = sys.entity_data(&p1).expect("p1 should exist")
            {
                println!("okay; now at ({:.3} {:.3} {:.3})", x1, y1, z1);
            }

            if let Point::In3d {
                coords: [x2, y2, z2],
                ..
            } = sys.entity_data(&p2).expect("p2 should exist")
            {
                println!("             ({:.3} {:.3} {:.3})", x2, y2, z2);
            }

            println!("{} DOF", dof);
        }
        SolveResult::Fail { .. } => println!("solve failed"),
    }
}
