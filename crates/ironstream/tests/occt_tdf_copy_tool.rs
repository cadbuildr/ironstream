use ironstream::tdf_copy_tool::*;

#[test]
fn reloc_table_bind_find() {
    let mut r = TdfRelocationTable::new();
    r.bind_label(10, 100);
    r.bind_attr(20, 200);
    assert_eq!(r.find_label(10), Some(100));
    assert_eq!(r.find_attr(20), Some(200));
    assert!(!r.is_bound_label(99));
}

#[test]
fn reloc_table_override() {
    let mut r = TdfRelocationTable::new();
    r.bind_label(5, 50);
    r.bind_label(5, 55);
    assert_eq!(r.find_label(5), Some(55));
    assert_eq!(r.nb_labels(), 1);
}

#[test]
fn reloc_table_clear() {
    let mut r = TdfRelocationTable::new();
    r.bind_label(1, 10);
    r.bind_attr(2, 20);
    r.clear();
    assert_eq!(r.nb_labels(), 0);
    assert_eq!(r.nb_attrs(), 0);
}

#[test]
fn copy_label_perform() {
    let mut r = TdfRelocationTable::new();
    let mut c = TdfCopyLabel::new(1, 100);
    assert!(!c.is_done());
    c.perform(&mut r);
    assert!(c.is_done());
    assert_eq!(r.find_label(1), Some(100));
}

#[test]
fn copy_tool_all_done() {
    let mut r = TdfRelocationTable::new();
    let mut t = TdfCopyTool::new();
    t.add(1, 10);
    t.add(2, 20);
    assert_eq!(t.nb_copies(), 2);
    assert!(!t.all_done());
    t.perform(&mut r);
    assert!(t.all_done());
    assert_eq!(r.nb_labels(), 2);
}

#[test]
fn reloc_table_self_relocation_flag() {
    let mut r = TdfRelocationTable::new();
    assert!(!r.self_relocation);
    r.set_self_relocation(true);
    assert!(r.self_relocation);
}
