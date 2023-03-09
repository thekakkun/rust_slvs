pub mod slvs;
use slvs::bindings;

fn main() {
    let g: bindings::Slvs_hGroup = 1;
    let x1 = bindings::Slvs_Param {
        h: 1,
        group: g,
        val: 10.0,
    };
    let y1 = bindings::Slvs_Param {
        h: 2,
        group: g,
        val: 10.0,
    };
    let z1 = bindings::Slvs_Param {
        h: 3,
        group: g,
        val: 10.0,
    };
    let p1 = bindings::Slvs_Entity {
        h: 101,
        group: g,
        type_: bindings::SLVS_E_POINT_IN_3D as i32,
        wrkpl: bindings::SLVS_FREE_IN_3D,
        point: [0; 4],
        normal: 0,
        distance: 0,
        param: [1, 2, 3, 0],
    };

    let x2 = bindings::Slvs_Param {
        h: 4,
        group: g,
        val: 20.0,
    };
    let y2 = bindings::Slvs_Param {
        h: 5,
        group: g,
        val: 20.0,
    };
    let z2 = bindings::Slvs_Param {
        h: 6,
        group: g,
        val: 20.0,
    };
    let p2 = bindings::Slvs_Entity {
        h: 102,
        group: g,
        type_: bindings::SLVS_E_POINT_IN_3D as i32,
        wrkpl: bindings::SLVS_FREE_IN_3D,
        point: [0; 4],
        normal: 0,
        distance: 0,
        param: [4, 5, 6, 0],
    };

    let c1 = bindings::Slvs_Constraint {
        h: 1,
        group: g,
        type_: bindings::SLVS_C_PT_PT_DISTANCE as i32,
        wrkpl: bindings::SLVS_FREE_IN_3D,
        valA: 30.0,
        ptA: 101,
        ptB: 102,
        entityA: 0,
        entityB: 0,
        entityC: 0,
        entityD: 0,
        other: 0,
        other2: 0,
    };

    let mut param_list = vec![x1, y1, z1, x2, y2, z2];
    let mut entity_list = vec![p1, p2];
    let mut constraint_list = vec![c1];
    let mut failed_list = vec![0; 50];

    let mut sys = bindings::Slvs_System {
        param: param_list.as_mut_ptr(),
        params: param_list.len() as i32,
        entity: entity_list.as_mut_ptr(),
        entities: entity_list.len() as i32,
        constraint: constraint_list.as_mut_ptr(),
        constraints: constraint_list.len() as i32,
        dragged: [4, 5, 6, 0],
        calculateFaileds: 1,
        failed: failed_list.as_mut_ptr(),
        faileds: failed_list.len() as i32,
        dof: 0,
        result: 0,
    };

    unsafe {bindings::Slvs_Solve(&mut sys, g)}

    if sys.result == bindings::SLVS_RESULT_OKAY.try_into().unwrap() {
        println!("solved okay");
    }
}
