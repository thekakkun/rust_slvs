use slvs::{
    constraint::{EqualLengthLines, PointsCoincident, PtOnLine, PtPtDistance, Vertical},
    entity::{LineSegment, Normal, Point, Workplane},
    make_quaternion, System,
};

// Demo of drawing a triangle in 3-space, consisting of lines between:
// - two points in 2-space
// - two points in 3-space
// - a point in 2-space and a point in 3-space
// Constrained to be equalateral, and standing perpendicular to the workplane.
fn main() {
    let mut sys = System::new();
    let g1 = sys.add_group();

    let origin = sys
        .sketch(Point::new_in_3d(g1, [0.0; 3]))
        .expect("Origin created");
    let normal = sys
        .sketch(Normal::new_in_3d(
            g1,
            make_quaternion([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        ))
        .expect("normal created");
    let workplane = sys
        .sketch(Workplane::new(g1, origin, normal))
        .expect("Workplane created");

    let g2 = sys.add_group();
    let p1a = sys
        .sketch(Point::new_on_workplane(g2, workplane, [1.0, 2.0]))
        .expect("point in 2d created");
    let p1b = sys
        .sketch(Point::new_on_workplane(g2, workplane, [3.0, 4.0]))
        .expect("point in 2d created");
    let l1 = sys
        .sketch(LineSegment::new(g2, p1a, p1b))
        .expect("line between two 2d points created");

    let p2a = sys
        .sketch(Point::new_on_workplane(g2, workplane, [5.0, 6.0]))
        .expect("point in 2d created");
    let p2b = sys
        .sketch(Point::new_in_3d(g2, [7.0, 8.0, 9.0]))
        .expect("point in 3d created");
    let l2 = sys
        .sketch(LineSegment::new(g2, p2a, p2b))
        .expect("line between 2d and 3d point created");

    let p3a = sys
        .sketch(Point::new_in_3d(g2, [10.0, 11.0, 12.0]))
        .expect("point in 3d created");
    let p3b = sys
        .sketch(Point::new_in_3d(g2, [13.0, 14.0, 15.0]))
        .expect("point in 3d created");
    let l3 = sys
        .sketch(LineSegment::new(g2, p3a, p3b))
        .expect("line between two 3d points created");

    // The workplane needs to be specified when constraining
    // two points on a workplane to be coincident.
    sys.constrain(PointsCoincident::new(g2, p1b, p2a, Some(workplane)))
        .expect("coincidence between two points in 2d");
    sys.constrain(PointsCoincident::new(g2, p2b, p3a, None))
        .expect("coincidence between two points in 3d");
    sys.constrain(PointsCoincident::new(g2, p3b, p1a, None))
        .expect("coincidence between 2d and 3d point");

    sys.constrain(PointsCoincident::new(g2, origin, p1a, Some(workplane)))
        .expect("p1a on origin");
    sys.constrain(Vertical::from_line(g2, workplane, l1))
        .expect("l1 is vertical on workplane");
    sys.constrain(PtPtDistance::new(g2, p1a, p2a, 100.0, None))
        .expect("l1 is 100 units long");
    sys.constrain(EqualLengthLines::new(g2, l1, l2, None))
        .expect("l1 and l2 are equal length");
    sys.constrain(EqualLengthLines::new(g2, l1, l3, None))
        .expect("l2 and l3 are equal length");
    sys.constrain(PtOnLine::new(g2, p3a, l1, Some(workplane)))
        .expect("p3a on l1 when projected onto workplane");

    let result = sys.solve(&g2);
    println!("{:#?}", result);
}
