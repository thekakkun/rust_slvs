use super::bindings;

pub struct Param {
    h: bindings::Slvs_hParam,
    group: bindings::Slvs_hGroup,
    val: f64,
}
