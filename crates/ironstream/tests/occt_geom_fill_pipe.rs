extern crate ironstream;
use ironstream::geom_fill_pipe::*;

#[test]
fn pipe_new_and_build() {
    let path = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let mut p = Pipe::new(path, 0.5);
    assert!(!p.is_done());
    p.build();
    assert!(p.is_done());
    assert!(p.error_on_surface() >= 0.0);
}

#[test]
fn pipe_sample_surface_count() {
    let path = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
    let mut p = Pipe::new(path, 1.0);
    p.build();
    let pts = p.sample_surface(3, 4);
    assert_eq!(pts.len(), 12);
}

#[test]
fn sweep_build_and_sections() {
    let path = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let mut s = Sweep::new(path);
    s.set_profile(vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
    s.set_tolerance(1e-5);
    s.set_continuity(1);
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_sections(), 3);
}

#[test]
fn location_guide_intervals() {
    let spine = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let guide = vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 1.0, 0.0]];
    let mut lg = LocationGuide::new(spine, guide);
    lg.set_contact(true);
    assert_eq!(lg.nb_intervals(), 2);
    assert!(lg.contact);
}

#[test]
fn section_generator_add() {
    let mut sg = SectionGenerator::new(3);
    sg.add_section(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    sg.add_section(vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
    assert_eq!(sg.nb_sections(), 2);
    assert_eq!(sg.n_sections, 3);
}
