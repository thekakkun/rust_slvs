use crate::binding;

pub type Entity = binding::Slvs_Entity;

impl From<Entity> for binding::Slvs_hEntity {
    fn from(value: Entity) -> Self {
        value.h
    }
}

pub enum EntityType {
    PointIn3d = binding::SLVS_E_POINT_IN_3D as _,
    PointIn2d = binding::SLVS_E_POINT_IN_2D as _,
    NormalIn3d = binding::SLVS_E_NORMAL_IN_3D as _,
    NormalIn2d = binding::SLVS_E_NORMAL_IN_2D as _,
    Distance = binding::SLVS_E_DISTANCE as _,
    Workplane = binding::SLVS_E_WORKPLANE as _,
    LineSegment = binding::SLVS_E_LINE_SEGMENT as _,
    Cubic = binding::SLVS_E_CUBIC as _,
    Circle = binding::SLVS_E_CIRCLE as _,
    ArcOfCircle = binding::SLVS_E_ARC_OF_CIRCLE as _,
}

pub trait AsEntity {
    fn type_(&self) -> EntityType;
    fn wrkpl(&self) -> Option<binding::Slvs_hEntity>;
    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4];
    fn normal(&self) -> Option<binding::Slvs_hEntity>;
    fn distance(&self) -> Option<binding::Slvs_hEntity>;
    fn param_vals(&self) -> [Option<f64>; 4];
}

pub struct PointIn3d {
    x: f64,
    y: f64,
    z: f64,
}

impl AsEntity for PointIn3d {
    fn type_(&self) -> EntityType {
        EntityType::PointIn3d
    }

    fn wrkpl(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [None; 4]
    }

    fn normal(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> [Option<f64>; 4] {
        [Some(self.x), Some(self.y), Some(self.z), None]
    }
}

struct LineSegment<'a> {
    pt_a: &'a Entity,
    pt_b: &'a Entity,
}

impl AsEntity for LineSegment<'_> {
    fn type_(&self) -> EntityType {
        EntityType::LineSegment
    }
    fn wrkpl(&self) -> Option<binding::Slvs_hEntity> {
        None
    }
    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [
            Some((*self.pt_a).into()),
            Some((*self.pt_b).into()),
            None,
            None,
        ]
    }
    fn normal(&self) -> Option<binding::Slvs_hEntity> {
        None
    }
    fn distance(&self) -> Option<binding::Slvs_hEntity> {
        None
    }
    fn param_vals(&self) -> [Option<f64>; 4] {
        [None; 4]
    }
}
