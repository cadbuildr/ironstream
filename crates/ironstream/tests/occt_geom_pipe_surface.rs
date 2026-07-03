// FILE: tests/occt_geom_pipe_surface.rs
//! Integration tests for `geom_pipe_surface` — the `BRepOffsetAPI_MakePipe` /
//! `GeomFill_Pipe` stub module.
//!
//! Tests exercise all public types:
//!   - [`PipeFillMode`]  — trihedron propagation mode enum
//!   - [`PipeParams`]    — construction parameters
//!   - [`PipeResult`]    — build-result container
//!   - [`GeomPipe`]      — the pipe-sweep builder

extern crate ironstream;

use ironstream::geom_pipe_surface::*;

// ---------------------------------------------------------------------------
// PipeFillMode tests
// ---------------------------------------------------------------------------

// ── test 1: all PipeFillMode variants are pairwise distinct ─────────────────
#[test]
fn test_fill_mode_all_variants_distinct() {
    let modes = [
        PipeFillMode::IsCorrectedFrenet,
        PipeFillMode::IsFixed,
        PipeFillMode::IsFrenet,
        PipeFillMode::IsConstantNormal,
        PipeFillMode::IsBiNormal,
        PipeFillMode::IsQuasiAngular,
        PipeFillMode::IsGuideAC,
        PipeFillMode::IsGuideACWithContact,
        PipeFillMode::IsGuidePlan,
        PipeFillMode::IsGuidePlanWithContact,
    ];
    for i in 0..modes.len() {
        for j in (i + 1)..modes.len() {
            assert_ne!(modes[i], modes[j], "modes[{i}] and modes[{j}] should be distinct");
        }
    }
}

// ── test 2: PipeFillMode implements Copy ────────────────────────────────────
#[test]
fn test_fill_mode_is_copy() {
    let a = PipeFillMode::IsFrenet;
    let b = a; // implicit copy
    assert_eq!(a, b);
}

// ── test 3: PipeFillMode implements Clone ────────────────────────────────────
#[test]
fn test_fill_mode_is_clone() {
    let a = PipeFillMode::IsCorrectedFrenet;
    let b = a.clone();
    assert_eq!(a, b);
}

// ---------------------------------------------------------------------------
// PipeParams tests
// ---------------------------------------------------------------------------

// ── test 4: PipeParams::new stores spine_length ──────────────────────────────
#[test]
fn test_pipe_params_spine_length() {
    let p = PipeParams::new(25.5, 4);
    assert_eq!(p.spine_length(), 25.5);
}

// ── test 5: PipeParams::new stores nb_profile_poles ─────────────────────────
#[test]
fn test_pipe_params_nb_profile_poles() {
    let p = PipeParams::new(1.0, 7);
    assert_eq!(p.nb_profile_poles, 7);
}

// ── test 6: PipeParams::new sets default fill_mode to IsFrenet ───────────────
#[test]
fn test_pipe_params_default_fill_mode_is_frenet() {
    let p = PipeParams::new(10.0, 3);
    assert_eq!(p.fill_mode(), PipeFillMode::IsFrenet);
}

// ── test 7: PipeParams::new sets is_solid to false ───────────────────────────
#[test]
fn test_pipe_params_default_not_solid() {
    let p = PipeParams::new(10.0, 3);
    assert!(!p.is_solid());
}

// ── test 8: PipeParams::set_fill_mode / fill_mode round-trips all variants ───
#[test]
fn test_pipe_params_set_fill_mode_all_variants() {
    let mut p = PipeParams::new(1.0, 2);
    for mode in [
        PipeFillMode::IsCorrectedFrenet,
        PipeFillMode::IsFixed,
        PipeFillMode::IsFrenet,
        PipeFillMode::IsConstantNormal,
        PipeFillMode::IsBiNormal,
        PipeFillMode::IsQuasiAngular,
        PipeFillMode::IsGuideAC,
        PipeFillMode::IsGuideACWithContact,
        PipeFillMode::IsGuidePlan,
        PipeFillMode::IsGuidePlanWithContact,
    ] {
        p.set_fill_mode(mode);
        assert_eq!(p.fill_mode(), mode, "fill_mode mismatch for {mode:?}");
    }
}

// ── test 9: PipeParams::set_solid / is_solid round-trip ─────────────────────
#[test]
fn test_pipe_params_set_solid() {
    let mut p = PipeParams::new(5.0, 4);
    assert!(!p.is_solid());
    p.set_solid(true);
    assert!(p.is_solid());
    p.set_solid(false);
    assert!(!p.is_solid());
}

// ── test 10: PipeParams::clone is independent of original ────────────────────
#[test]
fn test_pipe_params_clone_independence() {
    let p1 = PipeParams::new(50.0, 5);
    let mut p2 = p1.clone();

    p2.set_fill_mode(PipeFillMode::IsGuidePlan);
    p2.set_solid(true);

    // p1 must remain unchanged.
    assert_eq!(p1.fill_mode(), PipeFillMode::IsFrenet);
    assert!(!p1.is_solid());
    assert_eq!(p1.spine_length(), 50.0);
}

// ── test 11: PipeParams force_c1 field is accessible ────────────────────────
#[test]
fn test_pipe_params_force_c1() {
    let mut p = PipeParams::new(3.0, 2);
    assert!(!p.force_c1);
    p.force_c1 = true;
    assert!(p.force_c1);
}

// ---------------------------------------------------------------------------
// PipeResult tests
// ---------------------------------------------------------------------------

// ── test 12: PipeResult::new is not done ────────────────────────────────────
#[test]
fn test_pipe_result_new_not_done() {
    let r = PipeResult::new();
    assert!(!r.is_done());
}

// ── test 13: PipeResult::new has zero topology counts ───────────────────────
#[test]
fn test_pipe_result_new_zero_counts() {
    let r = PipeResult::new();
    assert_eq!(r.nb_shell_faces(), 0);
    assert_eq!(r.nb_wire_edges(), 0);
}

// ── test 14: PipeResult::build sets is_done ─────────────────────────────────
#[test]
fn test_pipe_result_build_is_done() {
    let mut r = PipeResult::new();
    r.build(6, 3);
    assert!(r.is_done());
}

// ── test 15: PipeResult::build stores face and edge counts ──────────────────
#[test]
fn test_pipe_result_build_stores_counts() {
    let mut r = PipeResult::new();
    r.build(12, 6);
    assert_eq!(r.nb_shell_faces(), 12);
    assert_eq!(r.nb_wire_edges(), 6);
}

// ── test 16: PipeResult::build can be called multiple times (last wins) ──────
#[test]
fn test_pipe_result_rebuild_overwrites() {
    let mut r = PipeResult::new();
    r.build(4, 2);
    r.build(20, 10);
    assert_eq!(r.nb_shell_faces(), 20);
    assert_eq!(r.nb_wire_edges(), 10);
    assert!(r.is_done());
}

// ── test 17: PipeResult::default matches PipeResult::new ────────────────────
#[test]
fn test_pipe_result_default_equals_new() {
    let r_default = PipeResult::default();
    let r_new = PipeResult::new();
    assert_eq!(r_default.is_done(), r_new.is_done());
    assert_eq!(r_default.nb_shell_faces(), r_new.nb_shell_faces());
    assert_eq!(r_default.nb_wire_edges(), r_new.nb_wire_edges());
}

// ── test 18: PipeResult is_done field is directly accessible ─────────────────
#[test]
fn test_pipe_result_is_done_field() {
    let r = PipeResult {
        is_done: true,
        nb_shell_faces: 2,
        nb_wire_edges: 1,
    };
    assert!(r.is_done());
    assert_eq!(r.nb_shell_faces(), 2);
    assert_eq!(r.nb_wire_edges(), 1);
}

// ---------------------------------------------------------------------------
// GeomPipe tests
// ---------------------------------------------------------------------------

// ── test 19: GeomPipe::new is not done before build ──────────────────────────
#[test]
fn test_geom_pipe_new_not_done() {
    let params = PipeParams::new(10.0, 4);
    let pipe = GeomPipe::new(params);
    assert!(!pipe.is_done());
}

// ── test 20: GeomPipe::result not done before build ──────────────────────────
#[test]
fn test_geom_pipe_result_not_done_before_build() {
    let params = PipeParams::new(10.0, 4);
    let pipe = GeomPipe::new(params);
    assert!(!pipe.result().is_done());
}

// ── test 21: GeomPipe::build marks is_done ───────────────────────────────────
#[test]
fn test_geom_pipe_build_marks_done() {
    let params = PipeParams::new(20.0, 5);
    let mut pipe = GeomPipe::new(params);
    pipe.build();
    assert!(pipe.is_done());
}

// ── test 22: GeomPipe::build sets nb_shell_faces = nb_profile_poles * 2 ──────
#[test]
fn test_geom_pipe_build_shell_faces() {
    for nb_poles in [1_usize, 3, 5, 8, 12] {
        let params = PipeParams::new(10.0, nb_poles);
        let mut pipe = GeomPipe::new(params);
        pipe.build();
        assert_eq!(
            pipe.result().nb_shell_faces(),
            nb_poles * 2,
            "nb_poles={nb_poles}"
        );
    }
}

// ── test 23: GeomPipe::build sets nb_wire_edges = nb_profile_poles ───────────
#[test]
fn test_geom_pipe_build_wire_edges() {
    for nb_poles in [1_usize, 4, 7, 10] {
        let params = PipeParams::new(10.0, nb_poles);
        let mut pipe = GeomPipe::new(params);
        pipe.build();
        assert_eq!(
            pipe.result().nb_wire_edges(),
            nb_poles,
            "nb_poles={nb_poles}"
        );
    }
}

// ── test 24: GeomPipe::result returns reference to internal result ────────────
#[test]
fn test_geom_pipe_result_borrow() {
    let nb_poles = 6_usize;
    let params = PipeParams::new(50.0, nb_poles);
    let mut pipe = GeomPipe::new(params);
    pipe.build();

    let res: &PipeResult = pipe.result();
    assert!(res.is_done());
    assert_eq!(res.nb_shell_faces(), nb_poles * 2);
    assert_eq!(res.nb_wire_edges(), nb_poles);
}

// ── test 25: GeomPipe params field is accessible after build ─────────────────
#[test]
fn test_geom_pipe_params_accessible_after_build() {
    let mut params = PipeParams::new(75.0, 9);
    params.set_fill_mode(PipeFillMode::IsBiNormal);
    params.set_solid(true);

    let mut pipe = GeomPipe::new(params);
    pipe.build();

    assert_eq!(pipe.params.spine_length(), 75.0);
    assert_eq!(pipe.params.nb_profile_poles, 9);
    assert_eq!(pipe.params.fill_mode(), PipeFillMode::IsBiNormal);
    assert!(pipe.params.is_solid());
}

// ── test 26: GeomPipe clone is independent ───────────────────────────────────
#[test]
fn test_geom_pipe_clone_independence() {
    let params = PipeParams::new(10.0, 4);
    let mut pipe1 = GeomPipe::new(params);
    let mut pipe2 = pipe1.clone();

    pipe1.build();
    // pipe2 should still not be done.
    assert!(!pipe2.is_done());

    pipe2.build();
    assert!(pipe2.is_done());
}

// ── test 27: GeomPipe with nb_profile_poles=1 produces 2 faces, 1 edge ───────
#[test]
fn test_geom_pipe_single_pole() {
    let params = PipeParams::new(1.0, 1);
    let mut pipe = GeomPipe::new(params);
    pipe.build();
    assert_eq!(pipe.result().nb_shell_faces(), 2);
    assert_eq!(pipe.result().nb_wire_edges(), 1);
}

// ── test 28: spine_length zero is accepted ───────────────────────────────────
#[test]
fn test_geom_pipe_zero_spine_length() {
    let params = PipeParams::new(0.0, 3);
    let mut pipe = GeomPipe::new(params);
    pipe.build();
    assert!(pipe.is_done());
    assert_eq!(pipe.params.spine_length(), 0.0);
}

// ── test 29: fill mode propagates from params into built pipe ─────────────────
#[test]
fn test_geom_pipe_fill_mode_propagates() {
    let mut params = PipeParams::new(5.0, 3);
    params.set_fill_mode(PipeFillMode::IsGuideACWithContact);

    let mut pipe = GeomPipe::new(params);
    pipe.build();

    assert_eq!(pipe.params.fill_mode(), PipeFillMode::IsGuideACWithContact);
    assert!(pipe.is_done());
}

// ── test 30: solid flag propagates from params into built pipe ────────────────
#[test]
fn test_geom_pipe_solid_flag_propagates() {
    let mut params = PipeParams::new(5.0, 3);
    params.set_solid(true);

    let mut pipe = GeomPipe::new(params);
    pipe.build();

    assert!(pipe.params.is_solid());
    assert!(pipe.is_done());
}
