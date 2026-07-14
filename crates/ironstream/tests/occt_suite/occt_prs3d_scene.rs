// FILE: tests/occt_prs3d_scene.rs
extern crate ironstream;
use ironstream::prs3d_scene::*;

#[test]
fn presentation_initial_state_erased() {
    let p = Presentation::new(1, 10, DisplayMode::Shaded);
    assert!(!p.is_displayed());
    assert!(!p.is_highlighted());
    assert_eq!(p.state, PresentationState::Erased);
}

#[test]
fn presentation_display_and_erase() {
    let mut p = Presentation::new(2, 5, DisplayMode::WireFrame);
    p.display();
    assert!(p.is_displayed());
    p.erase();
    assert!(!p.is_displayed());
}

#[test]
fn presentation_highlight_unhighlight() {
    let mut p = Presentation::new(3, 7, DisplayMode::Shaded);
    p.display();
    p.highlight();
    assert!(p.is_highlighted());
    p.unhighlight();
    assert!(!p.is_highlighted());
    assert!(p.is_displayed());
}

#[test]
fn presentation_manager_add_display_erase() {
    let mut mgr = PresentationManager::new();
    let id = mgr.add(100, DisplayMode::Shaded);
    mgr.display(id);
    assert_eq!(mgr.nb_displayed(), 1);
    mgr.erase(id);
    assert_eq!(mgr.nb_displayed(), 0);
}

#[test]
fn presentation_manager_erase_all() {
    let mut mgr = PresentationManager::new();
    let id1 = mgr.add(1, DisplayMode::Shaded);
    let id2 = mgr.add(2, DisplayMode::WireFrame);
    mgr.display(id1);
    mgr.display(id2);
    assert_eq!(mgr.nb_displayed(), 2);
    mgr.erase_all();
    assert_eq!(mgr.nb_displayed(), 0);
}

#[test]
fn scene_builder_wireframe() {
    let wf = SceneBuilder::wireframe(50);
    assert!(wf.is_displayed());
    assert_eq!(wf.nb_edges, 50);
    assert_eq!(wf.display_mode, DisplayMode::WireFrame);
}

#[test]
fn scene_builder_shaded_coverage_gt_wireframe() {
    let wf = SceneBuilder::wireframe(100);
    let sh = SceneBuilder::shaded(100);
    // Shaded: nb_triangles * 3 vs wireframe: nb_edges; so 300 > 100
    assert!(SceneBuilder::coverage(&sh) > SceneBuilder::coverage(&wf));
}
