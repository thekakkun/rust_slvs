use serde::{Deserialize, Serialize};

use super::{AsCubic, AsCurve, AsEntityData, EntityHandle, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_CUBIC},
    element::{AsHandle, TypeInfo},
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Cubic<T: AsTarget> {
    pub group: Group,
    pub workplane: Option<EntityHandle<Workplane>>,
    pub start_point: EntityHandle<Point<T>>,
    pub start_control: EntityHandle<Point<T>>,
    pub end_control: EntityHandle<Point<T>>,
    pub end_point: EntityHandle<Point<T>>,
}

impl Cubic<OnWorkplane> {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        start_point: EntityHandle<Point<OnWorkplane>>,
        start_control: EntityHandle<Point<OnWorkplane>>,
        end_control: EntityHandle<Point<OnWorkplane>>,
        end_point: EntityHandle<Point<OnWorkplane>>,
    ) -> Self {
        Self {
            group,
            workplane: Some(workplane),
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }
}

impl Cubic<In3d> {
    pub fn new(
        group: Group,
        start_point: EntityHandle<Point<In3d>>,
        start_control: EntityHandle<Point<In3d>>,
        end_control: EntityHandle<Point<In3d>>,
        end_point: EntityHandle<Point<In3d>>,
    ) -> Self {
        Self {
            group,
            workplane: None,
            start_point,
            start_control,
            end_control,
            end_point,
        }
    }
}

impl<T: AsTarget> AsCubic for Cubic<T> {}
impl<T: AsTarget> AsCurve for Cubic<T> {}

impl<T: AsTarget> AsEntityData for Cubic<T> {
    fn type_(&self) -> i32 {
        SLVS_E_CUBIC as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![
            self.start_point.handle(),
            self.start_control.handle(),
            self.end_control.handle(),
            self.end_point.handle(),
        ])
    }
}

impl<T: AsTarget> TypeInfo for Cubic<T> {
    fn type_of() -> String {
        format!("Cubic<{}>", T::type_of())
    }
}

impl<T: AsTarget> From<Slvs_Entity> for Cubic<T> {
    fn from(value: Slvs_Entity) -> Self {
        Self {
            group: Group(value.group),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(EntityHandle::new(h)),
            },
            start_point: EntityHandle::new(value.point[0]),
            start_control: EntityHandle::new(value.point[1]),
            end_control: EntityHandle::new(value.point[2]),
            end_point: EntityHandle::new(value.point[3]),
        }
    }
}
