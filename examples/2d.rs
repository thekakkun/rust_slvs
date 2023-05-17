use slvs::{
    constraint::{Diameter, EqualRadius, LineVertical, PtLineDistance, PtPtDistance},
    entity::{ArcOfCircle, Circle, Distance, LineSegment, Normal, Point, Workplane},
    make_quaternion,
    solver::FailReason,
    target::{In3d, OnWorkplane},
    System,
};

// An example of a constraint in 2d. In our first group, we create a workplane
// along the reference frame's xy plane. In a second group, we create some
// entities in that group and dimension them.
fn main() {
    let mut sys = System::new();
    let g1 = sys.add_group();

    // First, we create our workplane. Its origin corresponds to the origin
    // of our base frame (x y z) = (0 0 0)
    let origin = sys
        .sketch(Point::<In3d>::new(g1, 0.0, 0.0, 0.0))
        .expect("origin point created");

    // and it is parallel to the xy plane, so it has basis vectors (1 0 0)
    // and (0 1 0).
    let normal = sys
        .sketch(Normal::new_in_3d(
            g1,
            make_quaternion([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        ))
        .expect("normal created");
    let workplane = sys
        .sketch(Workplane::new(g1, origin, normal))
        .expect("workplane created");

    // Now create a second group. We'll solve group 2, while leaving group 1
    // constant; so the workplane that we've created will be locked down,
    // and the solver can't move it.
    let g2 = sys.add_group();

    // These points are represented by their coordinates (u v) within the
    // workplane, so they need only two parameters each.
    let p1 = sys
        .sketch(Point::<OnWorkplane>::new(g2, workplane, 10.0, 20.0))
        .expect("point in 2d created");
    let p2 = sys
        .sketch(Point::<OnWorkplane>::new(g2, workplane, 20.0, 10.0))
        .expect("point in 2d created");
    // And we create a line segment with those endpoints.
    let line = sys
        .sketch(LineSegment::<OnWorkplane>::new(g2, workplane, p1, p2))
        .expect("line segment created");

    // Now three more points.
    let arc_center = sys
        .sketch(Point::<OnWorkplane>::new(g2, workplane, 100.0, 120.0))
        .expect("point in 2d created");
    let arc_start = sys
        .sketch(Point::<OnWorkplane>::new(g2, workplane, 120.0, 110.0))
        .expect("point in 2d created");
    let arc_finish = sys
        .sketch(Point::<OnWorkplane>::new(g2, workplane, 115.0, 115.0))
        .expect("point in 2d created");
    // And arc, centered at point arc_center, starting at point arc_start, ending at
    // point arc_finish.
    let arc = sys
        .sketch(ArcOfCircle::new(
            g2, workplane, arc_center, arc_start, arc_finish, normal,
        ))
        .expect("arc created");

    // Now one more point, and a distance
    let circle_center = sys
        .sketch(Point::<OnWorkplane>::new(g2, workplane, 200.0, 200.0))
        .expect("point in 2d created");
    let circle_radius = sys
        .sketch(Distance::<OnWorkplane>::new(g2, workplane, 30.0))
        .expect("distance created");
    // And a complete circle, centered at point circle_center with radius equal to
    // distance circle_radius. The normal is the same as our workplane.
    let circle = sys
        .sketch(Circle::<OnWorkplane>::new(
            g2,
            workplane,
            circle_center,
            circle_radius,
            normal,
        ))
        .expect("circle created");

    // The length of our line segment is 30.0 units.
    sys.constrain(PtPtDistance::new(g2, p1, p2, 30.0, Some(workplane)))
        .expect("constrain line segment to 30.0 units");
    // And the distance from our line segment to the origin is 10.0 units.
    sys.constrain(PtLineDistance::new(g2, origin, line, 10.0, Some(workplane)))
        .expect("distance from line to origin is 10.0");
    // And the line segment is vertical.
    sys.constrain(LineVertical::new(g2, workplane, line))
        .expect("line segment is vertical");
    // And the distance from one endpoint to the origin is 15.0 units.
    sys.constrain(PtPtDistance::new(g2, p1, origin, 15.0, Some(workplane)))
        .expect("distance from p1 to origin is 15.0 units");
    // And same for the other endpoint; so if you add this constraint then
    // the sketch is overconstrained and will signal an error.
    // sys.constrain(PtPtDistance::new(g2, p2, origin, 18.0, Some(workplane)))
    //     .expect("distance from p2 to origin is 18.0 units");

    // The arc and the circle have equal radius.
    sys.constrain(EqualRadius::new(g2, arc, circle))
        .expect("arc and circle have equal radius");
    // The arc has radius 17.0 units.
    sys.constrain(Diameter::new(g2, arc, 17.0 * 2.0))
        .expect("arc has diameter of 17.0 units");

    // And solve.
    let result = sys.solve(&g2);

    if let Ok(ok_result) = result {
        println!("solved okay");
        let OnWorkplane(u1, v1) = sys.entity_data(&p1).expect("data for p1 found").coords;
        let OnWorkplane(u2, v2) = sys.entity_data(&p2).expect("data for p1 found").coords;
        println!("line from ({:.3} {:.3}) to ({:.3} {:.3})", u1, v1, u2, v2);

        let OnWorkplane(arc_center_u, arc_center_v) = sys
            .entity_data(&arc_center)
            .expect("data for arc_center found")
            .coords;
        let OnWorkplane(arc_start_u, arc_start_v) = sys
            .entity_data(&arc_start)
            .expect("data for arc_start found")
            .coords;
        let OnWorkplane(arc_finish_u, arc_finish_v) = sys
            .entity_data(&arc_finish)
            .expect("data for arc_finish found")
            .coords;
        println!(
            "arc center ({:.3} {:.3}) start ({:.3} {:.3}) finish ({:.3} {:.3})",
            arc_center_u, arc_center_v, arc_start_u, arc_start_v, arc_finish_u, arc_finish_v
        );

        let OnWorkplane(center_u, center_v) = sys
            .entity_data(&circle_center)
            .expect("data for circle_center found")
            .coords;
        let radius = sys
            .entity_data(&circle_radius)
            .expect("data for circle_radius found")
            .val;
        println!(
            "circle center ({:.3} {:.3}) radius {:.3}",
            center_u, center_v, radius
        );

        println!("{} DOF", ok_result.dof);
    } else if let Err(fail_result) = result {
        println!(
            "solve failed: problematic constraints are: {:#?}",
            fail_result.failed_constraints
        );

        match fail_result.reason {
            FailReason::Inconsistent => println!("system inconsistent"),
            _ => println!("system nonconvergent"),
        }
    }
}
