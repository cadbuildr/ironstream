// FILE: tests/occt_graphic3d_texture.rs
extern crate ironstream;
use ironstream::graphic3d_texture::*;

#[test]
fn texture_params_default_not_mipmapped() {
    let p = TextureParams::new();
    assert!(!p.is_mip_mapped());
    assert_eq!(p.filter, TextureFilter::Linear);
    assert_eq!(p.wrap_s, TextureWrap::Repeat);
}

#[test]
fn texture_params_mipmap_filter() {
    let p = TextureParams::new().with_filter(TextureFilter::LinearMipmapLinear);
    assert!(p.is_mip_mapped());
}

#[test]
fn texture_params_builder_chain() {
    let p = TextureParams::new()
        .with_wrap(TextureWrap::Clamp, TextureWrap::Clamp)
        .with_scale(2.0, 2.0);
    assert_eq!(p.wrap_s, TextureWrap::Clamp);
    assert!((p.scale[0] - 2.0).abs() < 1e-6);
}

#[test]
fn texture_root_from_file_valid() {
    let t = TextureRoot::from_file("albedo.png", 1);
    assert!(t.is_valid());
    assert_eq!(t.texture_id, 1);
}

#[test]
fn texture_root_from_memory_pixel_count() {
    let t = TextureRoot::from_memory(256, 128, 2);
    assert_eq!(t.nb_pixels(), 256 * 128);
    assert!(t.is_valid());
}

#[test]
fn texture2d_path_and_params() {
    let root = TextureRoot::from_file("normal.png", 3);
    let tex = Texture2D::new(root, TextureUnit::Normal);
    assert_eq!(tex.path(), "normal.png");
    assert!(tex.is_valid());
    assert_eq!(tex.unit, TextureUnit::Normal);
}

#[test]
fn texture_env_sphere_map() {
    let env = TextureEnv::sphere_map("env.hdr");
    assert!(env.is_valid());
    assert!(!env.is_cube_map);
}

#[test]
fn texture_env_cube_map_needs_six_faces() {
    let faces: Vec<String> = (0..6).map(|i| format!("face{}.png", i)).collect();
    let env = TextureEnv::cube_map(faces);
    assert!(env.is_valid());
    let env_bad = TextureEnv::cube_map(vec!["only_one.png".to_string()]);
    assert!(!env_bad.is_valid());
}
