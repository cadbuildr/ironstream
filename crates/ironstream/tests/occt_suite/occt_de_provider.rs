// FILE: tests/occt_de_provider.rs
extern crate ironstream;
use ironstream::de_provider::*;

// --- DeFormat ---

#[test]
fn de_format_name_roundtrip() {
    let formats = [
        DeFormat::Step,
        DeFormat::Iges,
        DeFormat::Gltf,
        DeFormat::Obj,
        DeFormat::Vrml,
        DeFormat::Stl,
        DeFormat::Brep,
        DeFormat::Unknown,
    ];
    for f in formats {
        assert_eq!(DeFormat::from_name(f.name()), f);
    }
}

#[test]
fn de_format_from_name_case_insensitive() {
    assert_eq!(DeFormat::from_name("step"), DeFormat::Step);
    assert_eq!(DeFormat::from_name("Step"), DeFormat::Step);
    assert_eq!(DeFormat::from_name("IGES"), DeFormat::Iges);
    assert_eq!(DeFormat::from_name("gltf"), DeFormat::Gltf);
}

#[test]
fn de_format_unknown_for_garbage_input() {
    assert_eq!(DeFormat::from_name("BOGUS"), DeFormat::Unknown);
    assert_eq!(DeFormat::from_name(""), DeFormat::Unknown);
}

#[test]
fn de_format_name_is_uppercase() {
    assert_eq!(DeFormat::Step.name(), "STEP");
    assert_eq!(DeFormat::Stl.name(), "STL");
}

// --- DeCapability ---

#[test]
fn de_capability_can_read_only() {
    let cap = DeCapability::CanRead;
    assert!(cap.can_read());
    assert!(!cap.can_write());
}

#[test]
fn de_capability_can_write_only() {
    let cap = DeCapability::CanWrite;
    assert!(!cap.can_read());
    assert!(cap.can_write());
}

#[test]
fn de_capability_can_read_write() {
    let cap = DeCapability::CanReadWrite;
    assert!(cap.can_read());
    assert!(cap.can_write());
}

// --- DeProviderParams ---

#[test]
fn params_new_defaults() {
    let p = DeProviderParams::new(DeFormat::Step, "stp");
    assert_eq!(p.format(), DeFormat::Step);
    assert_eq!(p.file_extension(), "stp");
    assert_eq!(p.encoding(), "UTF-8");
    assert!((p.tolerance() - 1e-7).abs() < 1e-20);
}

#[test]
fn params_set_tolerance() {
    let mut p = DeProviderParams::new(DeFormat::Stl, "stl");
    p.set_tolerance(1e-4);
    assert!((p.tolerance() - 1e-4).abs() < 1e-20);
}

#[test]
fn params_set_encoding() {
    let mut p = DeProviderParams::new(DeFormat::Iges, "igs");
    p.set_encoding("ASCII");
    assert_eq!(p.encoding(), "ASCII");
}

#[test]
fn params_format_and_extension_independent() {
    let p = DeProviderParams::new(DeFormat::Gltf, "glb");
    assert_eq!(p.format(), DeFormat::Gltf);
    assert_eq!(p.file_extension(), "glb");
}

// --- DeProvider ---

#[test]
fn provider_new_read_only() {
    let params = DeProviderParams::new(DeFormat::Step, "step");
    let prov = DeProvider::new("StepReader", params, DeCapability::CanRead);
    assert_eq!(prov.name(), "StepReader");
    assert_eq!(prov.format(), DeFormat::Step);
    assert!(prov.can_read());
    assert!(!prov.can_write());
}

#[test]
fn provider_new_write_only() {
    let params = DeProviderParams::new(DeFormat::Stl, "stl");
    let prov = DeProvider::new("StlWriter", params, DeCapability::CanWrite);
    assert!(!prov.can_read());
    assert!(prov.can_write());
}

#[test]
fn provider_new_read_write() {
    let params = DeProviderParams::new(DeFormat::Gltf, "gltf");
    let prov = DeProvider::new("GltfProvider", params, DeCapability::CanReadWrite);
    assert!(prov.can_read());
    assert!(prov.can_write());
    assert_eq!(prov.capability(), DeCapability::CanReadWrite);
}

#[test]
fn provider_params_accessible() {
    let params = DeProviderParams::new(DeFormat::Obj, "obj");
    let prov = DeProvider::new("ObjProvider", params, DeCapability::CanReadWrite);
    assert_eq!(prov.params().file_extension(), "obj");
    assert_eq!(prov.params().encoding(), "UTF-8");
}

#[test]
fn provider_params_mut_changes_tolerance() {
    let params = DeProviderParams::new(DeFormat::Brep, "brep");
    let mut prov = DeProvider::new("BRepProvider", params, DeCapability::CanReadWrite);
    prov.params_mut().set_tolerance(1e-3);
    assert!((prov.params().tolerance() - 1e-3).abs() < 1e-20);
}

// --- DeProviderRegistry ---

#[test]
fn registry_starts_empty() {
    let reg = DeProviderRegistry::new();
    assert_eq!(reg.nb_providers(), 0);
}

#[test]
fn registry_register_increases_count() {
    let mut reg = DeProviderRegistry::new();
    reg.register(DeProvider::new(
        "StepProvider",
        DeProviderParams::new(DeFormat::Step, "step"),
        DeCapability::CanReadWrite,
    ));
    assert_eq!(reg.nb_providers(), 1);
    reg.register(DeProvider::new(
        "StlProvider",
        DeProviderParams::new(DeFormat::Stl, "stl"),
        DeCapability::CanReadWrite,
    ));
    assert_eq!(reg.nb_providers(), 2);
}

#[test]
fn registry_find_by_format_returns_correct_provider() {
    let mut reg = DeProviderRegistry::new();
    reg.register(DeProvider::new(
        "StepProv",
        DeProviderParams::new(DeFormat::Step, "step"),
        DeCapability::CanReadWrite,
    ));
    reg.register(DeProvider::new(
        "GltfProv",
        DeProviderParams::new(DeFormat::Gltf, "gltf"),
        DeCapability::CanRead,
    ));

    let found = reg.find_by_format(DeFormat::Gltf).unwrap();
    assert_eq!(found.name(), "GltfProv");
}

#[test]
fn registry_find_by_format_missing_returns_none() {
    let reg = DeProviderRegistry::new();
    assert!(reg.find_by_format(DeFormat::Step).is_none());
}

#[test]
fn registry_find_by_extension_case_insensitive() {
    let mut reg = DeProviderRegistry::new();
    reg.register(DeProvider::new(
        "IgesProv",
        DeProviderParams::new(DeFormat::Iges, "IGS"),
        DeCapability::CanRead,
    ));
    // lowercase lookup
    let found = reg.find_by_extension("igs").unwrap();
    assert_eq!(found.format(), DeFormat::Iges);
}

#[test]
fn registry_find_by_extension_missing_returns_none() {
    let mut reg = DeProviderRegistry::new();
    reg.register(DeProvider::new(
        "StlProv",
        DeProviderParams::new(DeFormat::Stl, "stl"),
        DeCapability::CanWrite,
    ));
    assert!(reg.find_by_extension("step").is_none());
}

#[test]
fn registry_find_by_format_first_match_wins() {
    let mut reg = DeProviderRegistry::new();
    reg.register(DeProvider::new(
        "First",
        DeProviderParams::new(DeFormat::Step, "step"),
        DeCapability::CanRead,
    ));
    reg.register(DeProvider::new(
        "Second",
        DeProviderParams::new(DeFormat::Step, "stp"),
        DeCapability::CanWrite,
    ));
    let found = reg.find_by_format(DeFormat::Step).unwrap();
    assert_eq!(found.name(), "First");
}

#[test]
fn registry_multiple_formats_all_findable() {
    let mut reg = DeProviderRegistry::new();
    let formats = [
        (DeFormat::Step, "step"),
        (DeFormat::Iges, "igs"),
        (DeFormat::Gltf, "gltf"),
        (DeFormat::Obj, "obj"),
        (DeFormat::Vrml, "wrl"),
        (DeFormat::Stl, "stl"),
        (DeFormat::Brep, "brep"),
    ];
    for (fmt, ext) in &formats {
        reg.register(DeProvider::new(
            fmt.name(),
            DeProviderParams::new(*fmt, ext),
            DeCapability::CanReadWrite,
        ));
    }
    assert_eq!(reg.nb_providers(), formats.len());
    for (fmt, ext) in &formats {
        assert!(reg.find_by_format(*fmt).is_some());
        assert!(reg.find_by_extension(ext).is_some());
    }
}
