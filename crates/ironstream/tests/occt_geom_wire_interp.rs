extern crate ironstream;
use ironstream::geom_wire_interp::*;

#[test]
fn occt_wire_filling_smoke() {
    let mut wf = WireFilling::new(3, 9, 3);
    wf.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], 0);
    wf.add_wire(vec![[1.0, 0.0, 0.0], [0.5, 1.0, 0.0]], 0);
    wf.add_wire(vec![[0.5, 1.0, 0.0], [0.0, 0.0, 0.0]], 0);
    assert!(wf.is_done());
}

#[test]
fn occt_pipe_shell_ready() {
    let mut ps = PipeShell::new(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    ps.add_profile(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    assert!(ps.is_ready());
}
