use serde::{Deserialize, Serialize};

use super::{AsEntityData, EntityHandle, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_ARC_OF_CIRCLE},
    define_element,
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    group::Group,
    System,
};

define_element!(
    SLVS_E_ARC_OF_CIRCLE,
    /// An arc of a circle.
    ///
    /// An arc must always lie within a workplane; it cannot be free in 3d.
    /// So it is specified with a workplane.
    ///
    /// The arc runs counter-clockwise from its beginning to its end (with
    /// the workplane's normal pointing towards the viewer). If the beginning
    /// and end of the arc are coincident, then the arc is considered to
    /// represent a full circle.
    ///
    /// This representation has an extra degree of freedom. An extra
    /// constraint is therefore generated implicitly, so that
    ///
    /// ```text
    /// distance(center, start) = distance(center, end)
    /// ```
    ///
    /// This constraint is solved when adding the entity to the system, so points will
    /// be moved if an arc cannot be created from them.
    ///
    /// See the [module-level documentation][crate] for usage example.
    struct ArcOfCircle {
        /// The workpane Arc lies on
        workplane: EntityHandle<Workplane>,
        center: EntityHandle<Point>,
        /// The start point of the arc, going counter-clockwise from this point.
        arc_start: EntityHandle<Point>,
        /// The end point of the arc. Represents a full circle if coincident with
        /// the start point.
        arc_end: EntityHandle<Point>,
    }
);

impl AsEntityData for ArcOfCircle {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn points(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([
            self.center.handle(),
            self.arc_start.handle(),
            self.arc_end.handle(),
            0,
        ])
    }
}

impl FromSystem for ArcOfCircle {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_entity = sys.slvs_entity(element.handle())?;

        if SLVS_E_ARC_OF_CIRCLE == slvs_entity.type_ as _ {
            Ok(Self {
                group: Group(slvs_entity.group),
                workplane: EntityHandle::new(slvs_entity.wrkpl),
                center: EntityHandle::new(slvs_entity.point[0]),
                arc_start: EntityHandle::new(slvs_entity.point[1]),
                arc_end: EntityHandle::new(slvs_entity.point[2]),
            })
        } else {
            Err("Expected entity to have type SLVS_E_ARC_OF_CIRCLE.")
        }
    }
}
