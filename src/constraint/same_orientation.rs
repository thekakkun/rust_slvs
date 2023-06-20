use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_SAME_ORIENTATION},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, Normal},
    group::Group,
    System,
};

define_element!(
    SLVS_C_SAME_ORIENTATION,
    /// The normals `normal_a` and `normal_b` describe identical rotations.
    ///
    /// This constraint therefore restricts three degrees of freedom.
    /// 
    /// Note that this constraint in 3d space (`workplane` is `None`) is currently broken.
    struct SameOrientation {
        normal_a: EntityHandle<Normal>,
        normal_b: EntityHandle<Normal>,
    }
);

impl AsConstraintData for SameOrientation {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.normal_a.handle(), self.normal_b.handle(), 0, 0])
    }
}

impl FromSystem for SameOrientation {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_SAME_ORIENTATION == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                normal_a: EntityHandle::new(slvs_constraint.entityA),
                normal_b: EntityHandle::new(slvs_constraint.entityB),
            })
        } else {
            Err("Expected constraint to have type SLVS_C_SAME_ORIENTATION.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{constraint::SameOrientation, entity::Normal, utils::make_quaternion, System};

    #[test]
    #[ignore] // Crashes due to bug in original library.
    fn same_orientation() {
        let mut sys = System::new();

        let g = sys.add_group();
        let normal_a = sys
            .sketch(Normal::new_in_3d(
                g,
                make_quaternion([90.0, -15.0, -8.0], [-92.0, 65.0, 12.0]),
            ))
            .expect("normal created");
        let normal_b = sys
            .sketch(Normal::new_in_3d(
                g,
                make_quaternion([76.0, 34.0, -42.0], [78.0, 38.0, 22.0]),
            ))
            .expect("normal created");

        sys.constrain(SameOrientation::new(g, normal_a, normal_b))
            .expect("constraint added");
        dbg!(sys.solve(&g));

        if let (
            Normal::In3d {
                quaternion: quaternion_a,
                ..
            },
            Normal::In3d {
                quaternion: quaternion_b,
                ..
            },
        ) = (
            sys.entity_data(&normal_a).expect("data found"),
            sys.entity_data(&normal_b).expect("data found"),
        ) {
            assert_eq!(quaternion_a, quaternion_b);
        } else {
            unreachable!()
        }
    }
}
