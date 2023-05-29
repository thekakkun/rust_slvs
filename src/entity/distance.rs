use serde::{Deserialize, Serialize};

use super::AsEntityData;
use crate::{
    bindings::{Slvs_hGroup, SLVS_E_DISTANCE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

define_element!(
    SLVS_E_DISTANCE,
    /// An entity used to define a radius for [Circle][crate::entity::Circle].
    ///
    /// See the [module-level documentation][crate] for usage example.
    struct Distance {
        val: f64,
    }
);

impl AsEntityData for Distance {
    fn param_vals(&self) -> [Option<f64>; 4] {
        [Some(self.val), None, None, None]
    }
}

impl FromSystem for Distance {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;
        let distance_val = sys.slvs_param(slvs_entity.param[0])?.val;

        if SLVS_E_DISTANCE == slvs_entity.type_ as _ {
            Ok(Self {
                group: Group(slvs_entity.group),
                val: distance_val,
            })
        } else {
            Err("Expected entity to have type SLVS_E_DISTANCE.")
        }
    }
}
