use ironstream::de_format_ext::*;

#[test]
fn format_from_extension_known() {
    assert_eq!(DeFormat::from_extension("step"), DeFormat::Step);
    assert_eq!(DeFormat::from_extension("STP"), DeFormat::Step);
    assert_eq!(DeFormat::from_extension("IGES"), DeFormat::Iges);
    assert_eq!(DeFormat::from_extension("brep"), DeFormat::Brep);
    assert!(DeFormat::Step.is_known());
}

#[test]
fn format_from_extension_unknown() {
    assert_eq!(DeFormat::from_extension("xyz"), DeFormat::Unknown);
    assert!(!DeFormat::Unknown.is_known());
}

#[test]
fn format_from_path() {
    assert_eq!(DeFormat::from_path("model.brep"), DeFormat::Brep);
    assert_eq!(DeFormat::from_path("out.gltf"), DeFormat::Gltf);
    assert_eq!(DeFormat::from_path("part.obj"), DeFormat::Obj);
    assert_eq!(DeFormat::from_path("scan.stl"), DeFormat::Stl);
}

#[test]
fn wrapper_register_find_and_nb_providers() {
    let mut w = DeWrapper::new();
    w.register(DeProvider::new(DeFormat::Step, "StepProv"));
    w.register(DeProvider::new(DeFormat::Iges, "IgesProv"));
    assert_eq!(w.nb_providers(), 2);
    assert!(w.find(DeFormat::Step).is_some());
    assert!(w.find(DeFormat::Iges).is_some());
    assert!(w.find(DeFormat::Stl).is_none());
}

#[test]
fn wrapper_find_by_path() {
    let mut w = DeWrapper::new();
    w.register(DeProvider::new(DeFormat::Stl, "StlProv"));
    let p = w.find_by_path("mesh.stl");
    assert!(p.is_some());
    assert_eq!(p.unwrap().format, DeFormat::Stl);
    assert!(w.find_by_path("unknown.xyz").is_none());
}

#[test]
fn provider_stubs_and_shape_fix_params() {
    let p = DeProvider::new(DeFormat::Obj, "ObjProv");
    assert!(p.can_read());
    assert!(p.can_write());
    assert!(p.read("file.obj").is_ok());
    assert!(p.write("out.obj", 3).is_ok());

    let params = DeShapeFixParameters::new()
        .with_fix_tolerance(1e-4)
        .with_sew_shells(false);
    assert!(!params.sew_shells);
    assert!((params.fix_tolerance - 1e-4).abs() < 1e-15);
}
