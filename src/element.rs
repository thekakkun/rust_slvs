use std::fmt::Debug;

pub trait AsHandle: Debug {
    fn handle(&self) -> u32;
}

pub trait TypeInfo: Debug {
    fn type_of() -> String;
}
