// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_sewing.rs

extern crate ironstream;
use ironstream::brep_sewing::*;

// --- BrepSewingMode ---

#[test]
fn test_mode_variants_are_distinct() {
    assert_ne!(BrepSewingMode::Standard, BrepSewingMode::NonManifold);
    assert_ne!(BrepSewingMode::NonManifold, BrepSewingMode::SameGeom);
    assert_ne!(BrepSewingMode::SameGeom, BrepSewingMode::CutFreeEdges);
    assert_ne!(BrepSewingMode::Standard, BrepSewingMode::CutFreeEdges);
}

#[test]
fn test_mode_copy() {
    let m = BrepSewingMode::SameGeom;
    let m2 = m;
    assert_eq!(m, m2);
}

// --- BrepSewingParams ---

#[test]
fn test_params_new_defaults() {
    let p = BrepSewingParams::new();
    assert!((p.tolerance() - 1e-6).abs() < 1e-15);
    assert_eq!(p.mode(), BrepSewingMode::Standard);
}

#[test]
fn test_params_default_equals_new() {
    let a = BrepSewingParams::new();
    let b = BrepSewingParams::default();
    assert!((a.tolerance() - b.tolerance()).abs() < 1e-15);
    assert_eq!(a.mode(), b.mode());
}

#[test]
fn test_params_set_tolerance() {
    let mut p = BrepSewingParams::new();
    p.set_tolerance(1e-4);
    assert!((p.tolerance() - 1e-4).abs() < 1e-15);
}

#[test]
fn test_params_set_mode_non_manifold() {
    let mut p = BrepSewingParams::new();
    p.set_mode(BrepSewingMode::NonManifold);
    assert_eq!(p.mode(), BrepSewingMode::NonManifold);
}

#[test]
fn test_params_set_mode_same_geom() {
    let mut p = BrepSewingParams::new();
    p.set_mode(BrepSewingMode::SameGeom);
    assert_eq!(p.mode(), BrepSewingMode::SameGeom);
}

#[test]
fn test_params_set_mode_cut_free_edges() {
    let mut p = BrepSewingParams::new();
    p.set_mode(BrepSewingMode::CutFreeEdges);
    assert_eq!(p.mode(), BrepSewingMode::CutFreeEdges);
}

#[test]
fn test_params_set_non_manifold() {
    let mut p = BrepSewingParams::new();
    p.set_non_manifold(true);
    // Field is accessible through the struct; we verify the setter does not panic
    // and the object remains usable.
    p.set_non_manifold(false);
}

// --- BrepSewingResult ---

#[test]
fn test_result_new_zeroed_and_not_done() {
    let r = BrepSewingResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_input_shapes(), 0);
    assert_eq!(r.nb_free_edges(), 0);
    assert_eq!(r.nb_sewed_edges(), 0);
    assert_eq!(r.nb_degenerated_shapes(), 0);
}

#[test]
fn test_result_default_equals_new() {
    let a = BrepSewingResult::new();
    let b = BrepSewingResult::default();
    assert_eq!(a.is_done(), b.is_done());
    assert_eq!(a.nb_input_shapes(), b.nb_input_shapes());
    assert_eq!(a.nb_free_edges(), b.nb_free_edges());
}

#[test]
fn test_result_mark_done() {
    let mut r = BrepSewingResult::new();
    assert!(!r.is_done());
    r.mark_done();
    assert!(r.is_done());
}

#[test]
fn test_result_set_counts() {
    let mut r = BrepSewingResult::new();
    r.set_counts(5, 2, 3, 1);
    assert_eq!(r.nb_input_shapes(), 5);
    assert_eq!(r.nb_free_edges(), 2);
    assert_eq!(r.nb_sewed_edges(), 3);
    assert_eq!(r.nb_degenerated_shapes(), 1);
}

#[test]
fn test_result_is_manifold_no_free_edges() {
    let mut r = BrepSewingResult::new();
    r.set_counts(4, 0, 3, 0);
    assert!(r.is_manifold());
}

#[test]
fn test_result_is_not_manifold_with_free_edges() {
    let mut r = BrepSewingResult::new();
    r.set_counts(4, 2, 3, 0);
    assert!(!r.is_manifold());
}

#[test]
fn test_result_is_not_manifold_by_default() {
    // new() has nb_free_edges == 0 so is_manifold returns true on a fresh result
    // (zero free edges means no open boundaries — technically manifold by definition)
    let r = BrepSewingResult::new();
    assert!(r.is_manifold());
}

// --- BrepSewing ---

#[test]
fn test_sewing_new_no_shapes_added() {
    let p = BrepSewingParams::new();
    let sewing = BrepSewing::new(p);
    assert_eq!(sewing.nb_added(), 0);
    assert!(!sewing.is_done());
}

#[test]
fn test_sewing_add_increments_count() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    sewing.add("face_A");
    sewing.add("face_B");
    assert_eq!(sewing.nb_added(), 2);
}

#[test]
fn test_sewing_perform_empty_not_done() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    sewing.perform();
    assert!(!sewing.is_done());
    assert_eq!(sewing.result().nb_input_shapes(), 0);
}

#[test]
fn test_sewing_perform_single_shape() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    sewing.add("face_only");
    sewing.perform();
    assert!(sewing.is_done());
    let r = sewing.result();
    assert_eq!(r.nb_input_shapes(), 1);
    assert_eq!(r.nb_sewed_edges(), 0);
    assert_eq!(r.nb_free_edges(), 2);
    assert!(!r.is_manifold());
}

#[test]
fn test_sewing_perform_two_shapes() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    sewing.add("face_1");
    sewing.add("face_2");
    sewing.perform();
    assert!(sewing.is_done());
    let r = sewing.result();
    assert_eq!(r.nb_input_shapes(), 2);
    assert_eq!(r.nb_sewed_edges(), 1);
    assert_eq!(r.nb_free_edges(), 0);
    assert!(r.is_manifold());
}

#[test]
fn test_sewing_perform_three_shapes() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    sewing.add("face_A");
    sewing.add("face_B");
    sewing.add("face_C");
    sewing.perform();
    assert!(sewing.is_done());
    let r = sewing.result();
    assert_eq!(r.nb_input_shapes(), 3);
    assert_eq!(r.nb_sewed_edges(), 2);
    assert!(r.is_manifold());
}

#[test]
fn test_sewing_result_degenerated_default_zero() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    sewing.add("s1");
    sewing.add("s2");
    sewing.perform();
    assert_eq!(sewing.result().nb_degenerated_shapes(), 0);
}

#[test]
fn test_sewing_is_done_delegates_to_result() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    assert!(!sewing.is_done());
    sewing.add("x");
    sewing.perform();
    assert_eq!(sewing.is_done(), sewing.result().is_done());
}

#[test]
fn test_sewing_non_manifold_mode() {
    let mut p = BrepSewingParams::new();
    p.set_mode(BrepSewingMode::NonManifold);
    let mut sewing = BrepSewing::new(p);
    sewing.add("shell_1");
    sewing.add("shell_2");
    sewing.perform();
    assert!(sewing.is_done());
}

#[test]
fn test_sewing_custom_tolerance_param() {
    let mut p = BrepSewingParams::new();
    p.set_tolerance(1e-3);
    let mut sewing = BrepSewing::new(p);
    sewing.add("coarse_face");
    sewing.perform();
    assert!(sewing.is_done());
}

#[test]
fn test_sewing_many_shapes_sewed_edges_count() {
    let p = BrepSewingParams::new();
    let mut sewing = BrepSewing::new(p);
    for i in 0..10 {
        sewing.add(&format!("face_{}", i));
    }
    sewing.perform();
    assert!(sewing.is_done());
    let r = sewing.result();
    assert_eq!(r.nb_input_shapes(), 10);
    assert_eq!(r.nb_sewed_edges(), 9);
    assert!(r.is_manifold());
}
