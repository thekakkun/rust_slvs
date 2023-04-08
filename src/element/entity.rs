use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

static NEXT_ENTITY_H: AtomicU32 = AtomicU32::new(1);

pub(crate) enum EntityType {
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

pub type Entity = binding::Slvs_Entity;

impl Entity {
    pub(crate) fn new(
        group: binding::Slvs_hGroup,
        type_: EntityType,
        wrkpl: Option<binding::Slvs_hEntity>,
        point: [Option<binding::Slvs_hEntity>; 4],
        normal: Option<binding::Slvs_hEntity>,
        distance: Option<binding::Slvs_hEntity>,
        param: [Option<binding::Slvs_hParam>; 4],
    ) -> Self {
        Self {
            h: NEXT_ENTITY_H.fetch_add(1, Ordering::SeqCst),
            group,
            type_: type_ as i32,
            wrkpl: wrkpl.unwrap_or(0),
            point: [
                point[0].unwrap_or(0),
                point[1].unwrap_or(0),
                point[2].unwrap_or(0),
                point[3].unwrap_or(0),
            ],
            normal: normal.unwrap_or(0),
            distance: distance.unwrap_or(0),
            param: [
                param[0].unwrap_or(0),
                param[1].unwrap_or(0),
                param[2].unwrap_or(0),
                param[3].unwrap_or(0),
            ],
        }
    }
}
