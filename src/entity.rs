use super::bindings;

pub struct Entity {
    h: bindings::Slvs_hEntity,
    group: bindings::Slvs_hGroup,
    type_: i32,
    wrkpl: bindings::Slvs_hEntity,
    point: [bindings::Slvs_hEntity; 4],
    normal: bindings::Slvs_hEntity,
    distance: bindings::Slvs_hEntity,
    param: [bindings::Slvs_hEntity; 4],
}
