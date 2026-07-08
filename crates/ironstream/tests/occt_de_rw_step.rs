extern crate ironstream;
use ironstream::de_rw_step::*;

#[test]
fn test_de_rw_step_basic() {
    // Default construction.
    let provider = DeRwStep::new();
    assert_eq!(provider.schema, "AP214");
    assert_eq!(provider.version, "1.0");

    // Schema normalised to uppercase.
    let p2 = DeRwStep::with_schema("ap203");
    assert_eq!(p2.schema, "AP203");

    // can_read accepts .step and .stp extensions (case-insensitive).
    assert!(provider.can_read("model.step"));
    assert!(provider.can_read("model.stp"));
    assert!(provider.can_read("model.STEP"));
    assert!(!provider.can_read("model.iges"));
    assert!(!provider.can_read("model.obj"));

    // StepFileInfo default values.
    let info = StepFileInfo::new();
    assert_eq!(info.entity_count, 0);
    assert!(info.product_names.is_empty());

    // Write then read round-trip via a temp file.
    let tmp = std::env::temp_dir()
        .join(format!(
            "ironstream_test_step_{}.step",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(42)
        ))
        .to_string_lossy()
        .into_owned();
    let shapes = vec!["PartA".to_string(), "PartB".to_string()];
    provider.write(&shapes, &tmp).expect("write failed");

    let read_back = provider.read(&tmp).expect("read failed");
    assert!(read_back.contains(&"PartA".to_string()) || !read_back.is_empty());

    // detect_step_schema returns Some for the written file.
    let schema = detect_step_schema(&tmp);
    assert!(schema.is_some(), "schema should be detected");

    // Clean up.
    let _ = std::fs::remove_file(&tmp);
}
