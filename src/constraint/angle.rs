use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_ANGLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{EntityHandle, LineSegment, Workplane},
    group::Group,
    System,
};

define_element!(
    SLVS_C_ANGLE,
    /// The constrain the `angle` between `line_a` and `line_b`, in degrees.
    ///
    /// This constraint equation is written in the form
    ///
    /// ```text
    /// (A dot B)/(|A||B|) = cos(valA)
    /// ```
    ///
    /// where A and B are vectors in the directions of lines A and B. This equation
    /// does not specify the angle unambiguously; for example, note that `valA = +/- 90`
    /// degrees will produce the same equation.
    ///
    /// If `supplementary` is true, then the constraint is instead that
    ///
    /// ```text
    /// (A dot B)/(|A||B|) = -cos(valA)
    /// ```
    struct Angle {
        line_a: EntityHandle<LineSegment>,
        line_b: EntityHandle<LineSegment>,
        /// The angle between `line_a` and `line_b`, in degrees.
        angle: f64,
        /// If provided, constrains the angle between the lines projected onto this
        /// workplane.
        workplane: Option<EntityHandle<Workplane>>,
        /// If `true`, sets the supplementary angle.
        supplementary: bool,
    }
);

impl AsConstraintData for Angle {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.line_a.handle(), self.line_b.handle(), 0, 0])
    }

    fn val(&self) -> Option<f64> {
        Some(self.angle)
    }
}

impl FromSystem for Angle {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_ANGLE == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                line_a: EntityHandle::new(slvs_constraint.entityA),
                line_b: EntityHandle::new(slvs_constraint.entityB),
                angle: slvs_constraint.valA,
                supplementary: slvs_constraint.other != 0,
                workplane: match slvs_constraint.wrkpl {
                    0 => None,
                    h => Some(EntityHandle::new(h)),
                },
            })
        } else {
            Err("Expected constraint to have type SLVS_C_ANGLE.")
        }
    }
}
