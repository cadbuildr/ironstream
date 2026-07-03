use ironstream::geom_offset_ext::*;

#[test]
fn offset_continuity_default_is_g1() {
    assert_eq!(OffsetContinuity::default(), OffsetContinuity::G1);
}

#[test]
fn offset_surface_ext_value_stub() {
    let s = OffsetSurfaceExt::new(5, 3.0);
    let v = s.value(1.0, 2.0);
    // stub: z == offset
    assert!((v[2] - 3.0).abs() < 1e-10);
    assert!(!s.has_singular_points());
}

#[test]
fn offset_surface_ext_set_offset_updates_status() {
    let mut s = OffsetSurfaceExt::new(1, 5.0);
    s.set_offset(0.0);
    assert_eq!(s.status, OffsetSurfStatus::NoOffset);
    assert!(s.is_done()); // NoOffset counts as done
    s.set_offset(2.0);
    assert_eq!(s.status, OffsetSurfStatus::Done);
}

#[test]
fn geom_fill_pipe_with_section() {
    let mut pipe = GeomFillPipe::new_with_section(10, 20);
    assert!(!pipe.is_circular);
    pipe.generate_surface();
    assert!(pipe.is_done());
    assert_eq!(pipe.surface(), 10 + 5000);
    assert!((pipe.first_param() - 0.0).abs() < 1e-10);
    assert!((pipe.last_param() - 1.0).abs() < 1e-10);
}

#[test]
fn geom_fill_sweep_tolerance_set() {
    let mut sw = GeomFillSweep::new(3, 7);
    sw.set_tolerance(1e-3, 1e-2);
    assert!((sw.tolerance3d - 1e-3).abs() < 1e-15);
    assert!((sw.angle_tolerance - 1e-2).abs() < 1e-15);
    sw.build(0);
    assert!(sw.is_done());
}

#[test]
fn offset_make_result_brep_modes() {
    let r = OffsetMakeResult::done(42, BrepOffsetMode::Pipe, 1.5);
    assert!(r.is_done());
    assert_eq!(r.mode, BrepOffsetMode::Pipe);
    assert!((r.offset - 1.5).abs() < 1e-10);
    assert_eq!(r.shape_id, 42);
}
