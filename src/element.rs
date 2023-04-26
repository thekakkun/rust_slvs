use crate::bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D};

pub(super) struct Elements<T> {
    pub(super) list: Vec<T>,
    pub(super) next_h: u32,
}

impl<T> Elements<T> {
    pub(super) fn new() -> Self {
        Self {
            list: Vec::new(),
            next_h: 1,
        }
    }

    pub(super) fn get_next_h(&mut self) -> u32 {
        let current_h = self.next_h;
        self.next_h += 1;

        current_h
    }
}

impl<T> Default for Elements<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) trait AsHandle {
    fn as_handle(&self) -> u32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group(pub(super) u32);

impl AsHandle for Group {
    fn as_handle(&self) -> u32 {
        self.0
    }
}

pub trait AsTarget {
    fn type_() -> i32;
    fn as_vec(&self) -> Vec<f64>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_2D as _
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.0, self.1]
    }
}

impl From<Vec<f64>> for OnWorkplane {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_3D as _
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.0, self.1, self.2]
    }
}

impl From<Vec<f64>> for In3d {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1], value[2])
    }
}
