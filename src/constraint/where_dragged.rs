use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_WHERE_DRAGGED},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Point, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_WHERE_DRAGGED,
    /// The `point` is locked at its initial numerical guess and cannot be moved.
    ///
    /// This constrains two degrees of freedom in a workplane, and three in free space.
    /// It is therefore possible for this constraint to over-constrain the sketch,
    /// for example if it's applied to a point with one remaining degree of freedom.
    struct WhereDragged {
        point: EntityHandle<Point>,
        /// If provided, constraint applies when projected onto this workplane.
        workplane: Option<EntityHandle<Workplane>>,
    }
);
impl AsConstraintData for WhereDragged {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 2]> {
        Some([self.point.handle(), 0])
    }
}

impl FromSystem for WhereDragged {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_WHERE_DRAGGED == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                point: EntityHandle::new(slvs_constraint.ptA),
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_WHERE_DRAGGED.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint::{PtPtDistance, WhereDragged},
        entity::Point,
        len_within_tolerance,
        utils::distance,
        System,
    };

    #[test]
    fn where_dragged() {
        let mut sys = System::new();

        let g = sys.add_group();

        let initial_coords = [-87.0, 57.0, -32.0];

        let point_a = sys
            .sketch(Point::new_in_3d(g, initial_coords))
            .expect("point created");
        let point_b = sys
            .sketch(Point::new_in_3d(g, [-14.0, 46.0, -54.0]))
            .expect("point created");

        sys.constrain(WhereDragged::new(g, point_a, None))
            .expect("constraint added");
        sys.constrain(PtPtDistance::new(g, point_a, point_b, 10.0, None))
            .expect("constraint added");

        dbg!(sys.solve(&g));

        if let Point::In3d { coords, .. } = sys.entity_data(&point_a).expect("data found") {
            len_within_tolerance!(distance(initial_coords, coords), 0.0);
        } else {
            unreachable!()
        }
    }
}
