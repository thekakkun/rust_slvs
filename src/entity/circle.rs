use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::{AsEntityData, Distance, EntityHandle, Normal, Point, Workplane};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_E_CIRCLE},
    element::AsHandle,
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
    System,
};

/// A circle.
///
/// See the [module-level documentation][crate] for usage examples.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circle<T: AsTarget> {
    pub group: Group,
    pub workplane: Option<EntityHandle<Workplane>>,

    /// The center of the circle.
    pub center: EntityHandle<Point<T>>,

    /// The radius of the circle is defined by a handle to the [Distance] entity.
    pub radius: EntityHandle<Distance<T>>,
    /// The normal for the circle. This should be the normal for the workplane
    /// if the circle lies on a workplane.
    pub normal: EntityHandle<Normal>,
}

impl Circle<OnWorkplane> {
    /// Constructs a new `Circle` on a workplane.
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        center: EntityHandle<Point<OnWorkplane>>,
        radius: EntityHandle<Distance<OnWorkplane>>,
        normal: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            workplane: Some(workplane),
            center,
            radius,
            normal,
        }
    }
}

impl Circle<In3d> {
    /// Constructs a new `Circle` in 3d space.
    pub fn new(
        group: Group,
        center: EntityHandle<Point<In3d>>,
        radius: EntityHandle<Distance<In3d>>,
        normal: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            workplane: None,
            center,
            radius,
            normal,
        }
    }
}

impl<T: AsTarget> AsEntityData for Circle<T> {
    fn from_system(sys: &System, entity_handle: &EntityHandle<Self>) -> Result<Self, &'static str> {
        let slvs_entity = sys.slvs_entity(entity_handle.handle())?;

        Ok(Self {
            group: Group(slvs_entity.group),
            workplane: match slvs_entity.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
            center: EntityHandle::new(slvs_entity.point[0]),
            radius: EntityHandle::new(slvs_entity.distance),
            normal: EntityHandle::new(slvs_entity.normal),
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_E_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.center.handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        Some(self.radius.handle())
    }
}
