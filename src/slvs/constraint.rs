use super::bindings;

pub struct Constraint {
    h: bindings::Slvs_hConstraint,
    group: bindings::Slvs_hGroup,
    type_: i32,
    wrkpl: bindings::Slvs_hEntity,
    valA: f64,
    ptA: bindings::Slvs_hEntity,
    ptB: bindings::Slvs_hEntity,
    entityA: bindings::Slvs_hEntity,
    entityB: bindings::Slvs_hEntity,
    entityC: bindings::Slvs_hEntity,
    entityD: bindings::Slvs_hEntity,
    other: i32,
    other2: i32,
}
