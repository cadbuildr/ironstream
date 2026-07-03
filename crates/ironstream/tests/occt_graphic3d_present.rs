use ironstream::graphic3d_present::*;

#[test]
fn graphic3d_group_new_group_display_highlight() {
    let mut p = Graphic3dPresentation::new(1, DisplayMode::Shading);
    p.new_group();
    p.new_group();
    assert_eq!(p.nb_groups(), 2);
    p.display();
    assert!(p.is_displayed());
    p.highlight();
    assert!(p.is_highlighted());
}

#[test]
fn clipping_planes_add_dedup_remove() {
    let mut p = Graphic3dPresentation::new(1, DisplayMode::WireFrame);
    p.add_clipping_plane(10);
    p.add_clipping_plane(20);
    p.add_clipping_plane(10); // duplicate — should be ignored
    assert_eq!(p.clipping_plane_ids.len(), 2);
    p.remove_clipping_plane(10);
    assert_eq!(p.clipping_plane_ids.len(), 1);
    assert!(!p.clipping_plane_ids.contains(&10));
}

#[test]
fn presentation_manager_find_two_presentations_by_id() {
    let mut mgr = PrsMgrPresentationManager::new();
    mgr.add(Graphic3dPresentation::new(1, DisplayMode::Shading));
    mgr.add(Graphic3dPresentation::new(2, DisplayMode::WireFrame));
    assert_eq!(mgr.nb_presentations(), 2);
    let p = mgr.find(2).unwrap();
    assert_eq!(p.display_mode(), DisplayMode::WireFrame);
    let p1 = mgr.find(1).unwrap();
    assert_eq!(p1.display_mode(), DisplayMode::Shading);
}

#[test]
fn nb_displayed_1_out_of_2() {
    let mut mgr = PrsMgrPresentationManager::new();
    let mut p1 = Graphic3dPresentation::new(1, DisplayMode::Shading);
    p1.display();
    mgr.add(p1);
    mgr.add(Graphic3dPresentation::new(2, DisplayMode::Shading));
    assert_eq!(mgr.nb_displayed(), 1);
}

#[test]
fn priority_ordering_high_default_low() {
    assert!(PresentationPriority::HIGH > PresentationPriority::DEFAULT);
    assert!(PresentationPriority::DEFAULT > PresentationPriority::LOW);
    assert!(PresentationPriority::HIGH.value() > 0);
    assert!(PresentationPriority::LOW.value() < 0);
}
