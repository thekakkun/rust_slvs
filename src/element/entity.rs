use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

use super::group::GroupH;

static NEXT_ENTITY_H: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Copy)]
pub struct EntityH(binding::Slvs_hEntity);

impl EntityH {
    fn new() -> Self {
        Self(NEXT_ENTITY_H.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for EntityH {
    fn default() -> Self {
        Self::new()
    }
}

impl From<EntityH> for binding::Slvs_hEntity {
    fn from(value: EntityH) -> Self {
        value.0
    }
}

enum EntityType {
    Undefined,
    PointIn3d = binding::SLVS_E_POINT_IN_3D as isize,
    PointIn2d = binding::SLVS_E_POINT_IN_2D as isize,
    NormalIn3d = binding::SLVS_E_NORMAL_IN_3D as isize,
    NormalIn2d = binding::SLVS_E_NORMAL_IN_2D as isize,
    Distance = binding::SLVS_E_DISTANCE as isize,
    Workplane = binding::SLVS_E_WORKPLANE as isize,
    LineSegment = binding::SLVS_E_LINE_SEGMENT as isize,
    Cubic = binding::SLVS_E_CUBIC as isize,
    Circle = binding::SLVS_E_CIRCLE as isize,
    ArcOfCircle = binding::SLVS_E_ARC_OF_CIRCLE as isize,
}

impl binding::Slvs_Entity {
    fn new(
        group: GroupH,
        type_: EntityType,
        wrkpl: Option<EntityH>,
        point: [Option<EntityH>; 4],
        normal: Option<EntityH>,
        distance: Option<EntityH>,
        param: [Option<EntityH>; 4],
    ) -> Self {
        Self {
            h: EntityH::default().into(),
            group: group.into(),
            type_: type_ as i32,
            wrkpl: wrkpl.unwrap_or(EntityH(0)).into(),
            point: [
                point[0].unwrap_or(EntityH(0)).into(),
                point[1].unwrap_or(EntityH(0)).into(),
                point[2].unwrap_or(EntityH(0)).into(),
                point[3].unwrap_or(EntityH(0)).into(),
            ],
            normal: normal.unwrap_or(EntityH(0)).into(),
            distance: distance.unwrap_or(EntityH(0)).into(),
            param: [
                param[0].unwrap_or(EntityH(0)).into(),
                param[1].unwrap_or(EntityH(0)).into(),
                param[2].unwrap_or(EntityH(0)).into(),
                param[3].unwrap_or(EntityH(0)).into(),
            ],
        }
    }
}
