pub mod slvs;
use std::rc::Rc;

use slvs::parameter;

fn main() {
    let group = Rc::new(slvs::Group(1));
    let x1 = parameter::Param::new(1, &group, 10.0);
    let y1 = parameter::Param::new(2, &group, 10.0);
    let z1 = parameter::Param::new(3, &group, 10.0);
}
