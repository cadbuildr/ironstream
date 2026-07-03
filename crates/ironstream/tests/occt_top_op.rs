extern crate ironstream;
use ironstream::top_op::*;

#[test]
fn vpoint_kind_unknown_is_default() {
    let kind = VPointKind::default();
    assert_eq!(kind, VPointKind::Unknown);
    let vp = VPointInter::new(0, [0.0, 0.0, 0.0]);
    assert_eq!(vp.kind, VPointKind::Unknown);
}

#[test]
fn vpoint_inter_pnt_and_tolerance() {
    let vp = VPointInter::new(3, [1.5, 2.5, -0.5]);
    let p = vp.pnt();
    assert!((p[0] - 1.5).abs() < 1e-10);
    assert!((p[1] - 2.5).abs() < 1e-10);
    assert!((p[2] - (-0.5)).abs() < 1e-10);
    assert!((vp.tolerance() - 1e-7).abs() < 1e-15);
}

#[test]
fn hbuilder_nb_vpoints_after_add_vpoint() {
    let mut hb = HBuilder::new(1, 2);
    assert_eq!(hb.nb_vpoints(), 0);
    hb.add_vpoint(VPointInter::new(0, [0.0, 0.0, 0.0]));
    assert_eq!(hb.nb_vpoints(), 1);
    hb.add_vpoint(VPointInter::new(1, [1.0, 0.0, 0.0]));
    hb.add_vpoint(VPointInter::new(2, [2.0, 0.0, 0.0]));
    assert_eq!(hb.nb_vpoints(), 3);
    // Verify retrieval by index
    let v = hb.vpoint(1).unwrap();
    assert!((v.point[0] - 1.0).abs() < 1e-10);
}

#[test]
fn hbuilder_section_with_no_vpoints() {
    let mut hb = HBuilder::new(5, 6);
    let sec = hb.section();
    // No vpoints means no section edges
    assert!(sec.is_empty());
    assert!(hb.is_done()); // perform was called implicitly
}

#[test]
fn solid_builder_init_solid_clears_state() {
    let mut sb = SolidBuilder::new();
    sb.perform(vec![10, 20, 30]);
    sb.add_solid(99);
    assert!(sb.is_done());
    assert_eq!(sb.nb_shells(), 3);
    assert_eq!(sb.nb_solids(), 1);
    // init_solid should clear and reset
    sb.init_solid();
    assert_eq!(sb.nb_solids(), 0);
    assert_eq!(sb.nb_shells(), 0);
    assert_eq!(sb.status, HBuilderStatus::NotDone);
    assert!(!sb.is_done());
}

#[test]
fn hbuilder_status_done_is_done_returns_true() {
    let status = HBuilderStatus::Done;
    assert!(status.is_done());
    assert!(!HBuilderStatus::NotDone.is_done());
    assert!(!HBuilderStatus::Aborted.is_done());
    assert!(!HBuilderStatus::Error.is_done());
}
