use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Point};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_CUBIC},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

define_element!(
    SLVS_E_CUBIC,
    /// A nonrational cubic Bezier segment.
    ///
    /// The curve has the equation:
    ///
    /// `p(t) = P0*(1 - t)^3 + 3*P1*(1 - t)^2*t + 3*P2*(1 - t)*t^2 + P3*t^3]`
    ///
    /// as `t` goes from 0 to 1.
    struct Cubic {
        start_point: EntityHandle<Point>,
        start_control: EntityHandle<Point>,
        end_control: EntityHandle<Point>,
        end_point: EntityHandle<Point>,
    }
);

impl AsEntityData for Cubic {
    fn points(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([
            self.start_point.handle(),
            self.start_control.handle(),
            self.end_control.handle(),
            self.end_point.handle(),
        ])
    }
}

impl FromSystem for Cubic {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;

        if SLVS_E_CUBIC == slvs_entity.type_ as _ {
            Ok(Self {
                group: Group(slvs_entity.group),
                start_point: EntityHandle::new(slvs_entity.point[0]),
                start_control: EntityHandle::new(slvs_entity.point[1]),
                end_control: EntityHandle::new(slvs_entity.point[2]),
                end_point: EntityHandle::new(slvs_entity.point[3]),
            })
        } else {
            Err("Expected entity to have type SLVS_E_CUBIC.")
        }
    }
}
