use ironstream::brep_gap_fill::*;

#[test]
fn gap_fill_status_default_is_not_done() {
    let s = GapFillStatus::default();
    assert_eq!(s, GapFillStatus::NotDone);
    assert!(!s.is_ok());
}

#[test]
fn free_edge_default_tolerance() {
    let e = FreeEdge::new(1, 10, 100, 101);
    assert!((e.tolerance - 1e-7).abs() < 1e-15);
    assert!(!e.is_degenerated);
}

#[test]
fn shape_analysis_free_bounds_closed_wire() {
    // A single edge whose first and last vertex are equal forms a closed wire
    let mut fb = ShapeAnalysisFreeBouns::new(42, 1e-5);
    let mut e = FreeEdge::new(7, 10, 200, 200);
    fb.perform(vec![e]);
    assert!(fb.is_done());
    // first == last vertex => closed wire
    assert_eq!(fb.nb_closed_wires(), 1);
    assert_eq!(fb.nb_open_wires(), 0);
}

#[test]
fn shape_analysis_free_bounds_open_wire() {
    let mut fb = ShapeAnalysisFreeBouns::new(1, 1e-4);
    fb.perform(vec![
        FreeEdge::new(1, 10, 100, 101),
        FreeEdge::new(2, 10, 101, 102),
    ]);
    assert!(fb.is_done());
    assert_eq!(fb.nb_free_edges(), 2);
    assert!(fb.free_edge(0).is_some());
    assert!(fb.free_edge(5).is_none());
}

#[test]
fn find_contigous_edges_multiple_pairs() {
    let mut finder = FindContigousEdges::new(1e-4);
    finder.add(1);
    finder.add(2);
    finder.add(3);
    // edge 10 (face 1) and edge 20 (face 2) share vertex 101
    finder.add_free_edge(FreeEdge::new(10, 1, 100, 101));
    finder.add_free_edge(FreeEdge::new(20, 2, 101, 102));
    // edge 30 (face 3) and edge 10 (face 1) share vertex 100
    finder.add_free_edge(FreeEdge::new(30, 3, 100, 103));
    finder.perform();
    assert!(finder.is_done());
    assert!(finder.nb_contigous_edges() >= 1);
    assert_eq!(finder.nb_shapes(), 3);
}

#[test]
fn gap_fill_result_max_gap_tracking() {
    let mut r = GapFillResult::new();
    r.add_stitch(1, 2, 0.005);
    r.add_stitch(3, 4, 0.010);
    r.add_stitch(5, 6, 0.001);
    assert!((r.max_gap - 0.010).abs() < 1e-12);
    assert_eq!(r.nb_stitched(), 3);
    assert!(r.is_fully_stitched());
}
