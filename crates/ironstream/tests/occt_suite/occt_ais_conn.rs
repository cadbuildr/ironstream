extern crate ironstream;
use ironstream::ais_conn::*;

#[test]
fn conn_status_default_is_not_connected() {
    let status = ConnectStatus::default();
    assert_eq!(status, ConnectStatus::NotConnected);
}

#[test]
fn conn_transform_translation_part() {
    let t = ConnTransform::translation(3.0, 4.0, 5.0);
    let part = t.translation_part();
    assert!((part[0] - 3.0).abs() < 1e-10);
    assert!((part[1] - 4.0).abs() < 1e-10);
    assert!((part[2] - 5.0).abs() < 1e-10);
    assert!(!t.is_identity());
}

#[test]
fn ais_connected_has_owner_after_connect() {
    let mut obj = AisConnectedInteractive::new(7);
    assert!(!obj.has_owner());
    obj.connect(99);
    assert!(obj.has_owner());
    assert_eq!(obj.referenced_id, Some(99));
}

#[test]
fn ais_connected_erase_hides() {
    let mut obj = AisConnectedInteractive::new(2);
    obj.display();
    assert!(obj.is_displayed);
    obj.erase();
    assert!(!obj.is_displayed);
}

#[test]
fn ais_multiple_connected_referenced_ids() {
    let mut mc = AisMultipleConnected::new(5);
    assert!(mc.is_empty());
    mc.add_connection(10, ConnTransform::identity());
    mc.add_connection(20, ConnTransform::identity());
    let ids = mc.referenced_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&10));
    assert!(ids.contains(&20));
}

#[test]
fn ais_multiple_connected_display_erase() {
    let mut mc = AisMultipleConnected::new(3);
    mc.display();
    assert!(mc.is_displayed);
    mc.erase();
    assert!(!mc.is_displayed);
}
