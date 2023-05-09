/*!
This crate is a wrapper around the [SolveSpace](https://solvespace.com/index.pl)
geometic constraint solver library.

Sketch geometries by creating [entities][`entity`] within the system,
then add [constraints][`constraint`] to define relationships between multiple
entities.

While the original C++ library used the `Slvs_Entity` and `Slvs_Constraint` structs
to store all data pertaining to entities and constraints, these have been split
into handles ([`entity::EntityHandle`] and [`constraint::ConstraintHandle`]) and
data (structs that implement [`entity::AsEntityData`] and [`constraint::AsConstraintData`])
use the handles as "keys" to reference elements, and data structs as typed "values"
to add and update elements within `System`.

# Example: In 3d space.


```rust
// Initialize the system, and create a single [`group`].
let mut sys = System::new();
let g = sys.add_group();

// Create two [points][`entity::Point`]. The first at coordinates (10, 10, 10) and
// the second at (20, 20, 20).

let p1 = sys
    .sketch(&g, Point::<In3d>::new(10.0, 10.0, 10.0))
    .expect("p1 created");
let p2 = sys
    .sketch(&g, Point::<In3d>::new(20.0, 20.0, 20.0))
    .expect("p2 created");

//Draw a LineSegment connecting the two points.
sys.sketch(&g, LineSegment::<In3d>::new(p1, p2))
    .expect("line segment created");

// Constrain the  PtPtDistance between the two points to be 30 units.
sys.constrain(PtPtDistance::new(g, p1, p2, 30.0, None))

// Specifying set_dragged() on an entity tells the solver that the entity should
// be kept as close as possible to its initial location.
sys.set_dragged(&p2);

// And now we solve the system.
let result = sys.solve(&g);
sys.clear_dragged();


// If done correctly, the following should apply:
// - The distance between `p1` and `p2` should be 30.0 units
// - `p2` should still be placed near its initial location of (20, 20, 20)
if let Ok(ok_result) = result {
    let In3d(x1, y1, z1) = sys.entity_data(&p1).expect("p1 should exist").coords;
    println!("okay; now at ({:.3} {:.3} {:.3})", x1, y1, z1);

    let In3d(x2, y2, z2) = sys.entity_data(&p2).expect("p2 should exist").coords;
    println!("             ({:.3} {:.3} {:.3})", x2, y2, z2);

    println!("{} DOF", ok_result.dof);
} else {
    println!("solve failed");
}
```
*/

pub mod bindings;
pub use bindings::{make_quaternion, quaternion_n, quaternion_u, quaternion_v};

pub mod solver;

mod element;
pub mod group;

pub mod target;

pub mod entity;

pub mod constraint;

mod system;
pub use system::System;
