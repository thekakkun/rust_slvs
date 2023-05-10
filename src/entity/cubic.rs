use serde::{Deserialize, Serialize};

use super::{AsCubic, AsCurve, AsEntityData, EntityHandle, Point, SomeEntityHandle, Workplane};
use crate::{
    bindings::{
        Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_CUBIC, SLVS_E_POINT_IN_2D,
        SLVS_E_POINT_IN_3D,
    },
    element::AsHandle,
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
    fn into_some_entity_handle(handle: u32) -> SomeEntityHandle {
        match T::slvs_type() as _ {
            SLVS_E_POINT_IN_2D => {
                SomeEntityHandle::Cubic(CubicHandle::OnWorkplane(EntityHandle::new(handle)))
            }
            SLVS_E_POINT_IN_3D => {
                SomeEntityHandle::Cubic(CubicHandle::In3d(EntityHandle::new(handle)))
            }
            _ => panic!("Unknown slvs_type {}", T::slvs_type()),
        }
    }

    fn slvs_type(&self) -> i32 {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CubicHandle {
    OnWorkplane(EntityHandle<Cubic<OnWorkplane>>),
    In3d(EntityHandle<Cubic<In3d>>),
}

impl AsHandle for CubicHandle {
    fn handle(&self) -> u32 {
        match self {
            Self::OnWorkplane(entity_handle) => entity_handle.handle(),
            Self::In3d(entity_handle) => entity_handle.handle(),
        }
    }
}

impl TryFrom<Slvs_Entity> for CubicHandle {
    type Error = &'static str;

    fn try_from(value: Slvs_Entity) -> Result<Self, Self::Error> {
        if value.type_ == SLVS_E_CUBIC as _ {
            match value.wrkpl {
                0 => Ok(CubicHandle::In3d(value.into())),
                _ => Ok(CubicHandle::OnWorkplane(value.into())),
            }
        } else {
            Err("Unexpected Slvs_Entity type")
        }
    }
}
