extern crate ironstream;
use ironstream::prs_mgr::*;

#[test]
fn test_presentation_display_erase_cycle() {
    let mut p = Presentation::new(0, 0);
    assert!(!p.is_displayed);
    p.display();
    assert!(p.is_displayed);
    p.erase();
    assert!(!p.is_displayed);
}

#[test]
fn test_presentation_highlight_and_unhighlight() {
    let mut p = Presentation::new(1, 0);
    p.highlight(HighlightMode::Highlight);
    assert!(p.is_highlighted);
    assert_eq!(p.highlight_mode, HighlightMode::Highlight);
    p.unhighlight();
    assert!(!p.is_highlighted);
    assert_eq!(p.highlight_mode, HighlightMode::Normal);
}

#[test]
fn test_presentable_object_add_presentations() {
    let mut obj = PresentableObject::new(10);
    obj.add_presentation(0);
    obj.add_presentation(1);
    assert_eq!(obj.nb_presentations(), 2);
    assert!(obj.has_presentation(1));
    assert!(!obj.has_presentation(2));
}

#[test]
fn test_presentation_manager_display_and_query() {
    let mut mgr = PresentationManager::new();
    mgr.add_object(PresentableObject::new(1));
    assert!(mgr.display(1, 0));
    assert!(mgr.is_displayed(1, 0));
    assert_eq!(mgr.nb_displayed(), 1);
}

#[test]
fn test_presentation_manager_highlight_unhighlight() {
    let mut mgr = PresentationManager::new();
    mgr.add_object(PresentableObject::new(5));
    mgr.display(5, 0);
    mgr.highlight(5, 0);
    assert!(mgr.is_highlighted(5));
    mgr.unhighlight(5);
    assert!(!mgr.is_highlighted(5));
}

#[test]
fn test_presentation_manager_erase_removes_from_displayed() {
    let mut mgr = PresentationManager::new();
    mgr.add_object(PresentableObject::new(3));
    mgr.display(3, 0);
    mgr.erase(3, 0);
    assert!(!mgr.is_displayed(3, 0));
    assert_eq!(mgr.nb_displayed(), 0);
}
