use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_CURVE_CURVE_TANGENT},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsCurve, EntityHandle, Workplane},
    group::Group,
    System,
};

/// `curve_a` and `curve_b` are tangent.
///
/// These entities can either be an [`ArcOfCircle`][crate::entity::ArcOfCircle] or
/// [`Cubic`][crate::entity::Cubic], in any combination. A workplane to project onto is
/// required for all combination of entities.
///
/// `to_curve_a_end` and `to_curve_b_end` control which end of the curves the constraint
/// applies to.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    pub group: Group,
    pub workplane: EntityHandle<Workplane>,
    pub curve_a: EntityHandle<CA>,
    pub curve_b: EntityHandle<CB>,
    pub to_curve_a_end: bool,
    pub to_curve_b_end: bool,
}

impl<CA, CB> CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        curve_a: EntityHandle<CA>,
        curve_b: EntityHandle<CB>,
        to_curve_a_end: bool,
        to_curve_b_end: bool,
    ) -> Self {
        Self {
            group,
            workplane,
            curve_a,
            curve_b,
            to_curve_a_end,
            to_curve_b_end,
        }
    }
}

impl<CA, CB> AsGroup for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<CA, CB> AsSlvsType for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn slvs_type(&self) -> i32 {
        SLVS_C_CURVE_CURVE_TANGENT as _
    }
}

impl<CA, CB> AsConstraintData for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn entities(&self) -> Option<[Slvs_hEntity; 4]> {
        Some([self.curve_a.handle(), self.curve_b.handle(), 0, 0])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_curve_a_end, self.to_curve_b_end]
    }
}

impl<CA, CB> FromSystem for CurveCurveTangent<CA, CB>
where
    CA: AsCurve,
    CB: AsCurve,
{
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_CURVE_CURVE_TANGENT == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                workplane: EntityHandle::new(slvs_constraint.wrkpl),
                curve_a: EntityHandle::new(slvs_constraint.entityA),
                curve_b: EntityHandle::new(slvs_constraint.entityB),
                to_curve_a_end: slvs_constraint.other != 0,
                to_curve_b_end: slvs_constraint.other2 != 0,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_CURVE_CURVE_TANGENT.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        angle_within_tolerance,
        constraint::CurveCurveTangent,
        entity::{ArcOfCircle, Cubic, Normal, Point, Workplane},
        utils::{angle_2d, make_quaternion, project_on_plane, rounded_mod},
        System,
    };

    #[test]
    fn arc_arc_tangent() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-67.0, 18.0, 49.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([16.0, 62.0, -44.0], [-56.0, -68.0, 76.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();
        let center_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [49.0, 62.0]))
            .expect("point created");
        let arc_start_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-37.0, 93.0]))
            .expect("point created");
        let arc_end_a = sys
            .sketch(Point::new_on_workplane(g, workplane, [-4.0, -9.0]))
            .expect("point created");
        let arc_a = sys
            .sketch(ArcOfCircle::new(
                g,
                workplane,
                center_a,
                arc_start_a,
                arc_end_a,
            ))
            .expect("arc created");

        let center_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [-14.0, 72.0]))
            .expect("point created");
        let arc_start_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [59.0, -6.0]))
            .expect("point created");
        let arc_end_b = sys
            .sketch(Point::new_on_workplane(g, workplane, [29.0, -24.0]))
            .expect("point created");
        let arc_b = sys
            .sketch(ArcOfCircle::new(
                g,
                workplane,
                center_b,
                arc_start_b,
                arc_end_b,
            ))
            .expect("arc created");

        sys.constrain(CurveCurveTangent::new(
            g, workplane, arc_a, arc_b, false, false,
        ))
        .expect("constraint added");
        dbg!(sys.solve(&g));

        if let (
            Point::OnWorkplane {
                coords: center_a, ..
            },
            Point::OnWorkplane {
                coords: start_a, ..
            },
            Point::OnWorkplane {
                coords: center_b, ..
            },
            Point::OnWorkplane {
                coords: start_b, ..
            },
        ) = (
            sys.entity_data(&center_a).expect("data found"),
            sys.entity_data(&arc_start_a).expect("data found"),
            sys.entity_data(&center_b).expect("data found"),
            sys.entity_data(&arc_start_b).expect("data found"),
        ) {
            let angle = angle_2d([center_a, start_a], [center_b, start_b]);
            angle_within_tolerance!(rounded_mod(angle, 180.0), 0_f64);
        } else {
            unreachable!()
        };
    }

    #[test]
    fn cubic_cubic_tangent() {
        let mut sys = System::new();

        let workplane_g = sys.add_group();
        let origin = sys
            .sketch(Point::new_in_3d(workplane_g, [-35.0, -37.0, 49.0]))
            .expect("origin created");
        let normal = sys
            .sketch(Normal::new_in_3d(
                workplane_g,
                make_quaternion([-77.0, -51.0, 21.0], [-12.0, -84.0, 13.0]),
            ))
            .expect("normal created");
        let workplane = sys
            .sketch(Workplane::new(workplane_g, origin, normal))
            .expect("workplane created");

        let g = sys.add_group();

        let start_point_a = sys
            .sketch(Point::new_in_3d(g, [56.0, -98.0, -85.0]))
            .expect("point created");
        let start_control_a = sys
            .sketch(Point::new_in_3d(g, [-5.0, -62.0, 72.0]))
            .expect("point created");
        let end_control_a = sys
            .sketch(Point::new_in_3d(g, [45.0, 79.0, -39.0]))
            .expect("point created");
        let end_point_a = sys
            .sketch(Point::new_in_3d(g, [-2.0, -64.0, 34.0]))
            .expect("point created");
        let cubic_a = sys
            .sketch(Cubic::new(
                g,
                start_point_a,
                start_control_a,
                end_control_a,
                end_point_a,
            ))
            .expect("cubic created");

        let start_point_b = sys
            .sketch(Point::new_in_3d(g, [80.0, -54.0, -43.0]))
            .expect("point created");
        let start_control_b = sys
            .sketch(Point::new_in_3d(g, [-7.0, -64.0, 73.0]))
            .expect("point created");
        let end_control_b = sys
            .sketch(Point::new_in_3d(g, [92.0, 55.0, -41.0]))
            .expect("point created");
        let end_point_b = sys
            .sketch(Point::new_in_3d(g, [-61.0, 59.0, -12.0]))
            .expect("point created");
        let cubic_b = sys
            .sketch(Cubic::new(
                g,
                start_point_b,
                start_control_b,
                end_control_b,
                end_point_b,
            ))
            .expect("cubic created");

        sys.constrain(CurveCurveTangent::new(
            g, workplane, cubic_a, cubic_b, false, false,
        ))
        .expect("constraint added");
        dbg!(sys.solve(&g));

        if let (
            Point::In3d { coords: origin, .. },
            Normal::In3d { quaternion, .. },
            Point::In3d {
                coords: start_a, ..
            },
            Point::In3d {
                coords: control_a, ..
            },
            Point::In3d {
                coords: start_b, ..
            },
            Point::In3d {
                coords: control_b, ..
            },
        ) = (
            sys.entity_data(&origin).expect("data found"),
            sys.entity_data(&normal).expect("data found"),
            sys.entity_data(&start_point_a).expect("data found"),
            sys.entity_data(&start_control_a).expect("data found"),
            sys.entity_data(&start_point_b).expect("data found"),
            sys.entity_data(&start_control_b).expect("data found"),
        ) {
            let start_a = project_on_plane(start_a, origin, quaternion);
            let control_a = project_on_plane(control_a, origin, quaternion);
            let start_b = project_on_plane(start_b, origin, quaternion);
            let control_b = project_on_plane(control_b, origin, quaternion);

            let angle = angle_2d([start_a, control_a], [start_b, control_b]);

            angle_within_tolerance!(rounded_mod(angle, 180.0), 0_f64);
        } else {
            unreachable!()
        }
    }
}
