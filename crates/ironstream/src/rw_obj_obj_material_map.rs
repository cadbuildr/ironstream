// FILE: rw_obj_obj_material_map.rs
// occt: RWObj_ObjMaterialMap

//! Material MTL file writer for OBJ export.
//! Faithful port of `RWObj_ObjMaterialMap` (.hxx + .cxx) together with the
//! inherited `RWMesh_MaterialMap` registration behavior it relies on:
//! style deduplication in a double map, "mat_" key generation with
//! collision suffixing, lazy file opening on first AddMaterial, and the
//! MTL statements (newmtl/Ka/Kd/Ks/Ns/Tr/map_Kd) from DefineMaterial.
//! XCAF style/material/texture inputs are modeled as local records.

use std::rc::Rc;

/// Local stand-in for `XCAFDoc_VisMaterialCommon`.
#[derive(Clone, Debug, PartialEq)]
pub struct VisMaterialCommonOmm {
    /// TDataStd_Name of the material label (None -> fallback "mat").
    pub label_name: Option<String>,
    pub ambient: [f64; 3],
    pub diffuse: [f64; 3],
    pub specular: [f64; 3],
    pub shininess: f32,
    pub transparency: f32,
}

impl Default for VisMaterialCommonOmm {
    /// XCAFDoc_VisMaterialCommon defaults.
    fn default() -> Self {
        VisMaterialCommonOmm {
            label_name: None,
            ambient: [0.1, 0.1, 0.1],
            diffuse: [0.8, 0.8, 0.8],
            specular: [0.2, 0.2, 0.2],
            shininess: 1.0,
            transparency: 0.0,
        }
    }
}

/// Local stand-in for `Image_Texture` (handle identity matters).
#[derive(Debug)]
pub struct ImageTextureStubOmm {
    pub source_path: String,
}

pub type HandleImageTextureOmm = Rc<ImageTextureStubOmm>;

/// Local stand-in for `XCAFPrs_Style`.
#[derive(Clone, Debug, Default)]
pub struct XcafStyleStubOmm {
    pub material: Option<VisMaterialCommonOmm>,
    /// Surface color with alpha (IsSetColorSurf when Some).
    pub color_surf_rgba: Option<([f64; 3], f32)>,
    pub base_color_texture: Option<HandleImageTextureOmm>,
}

impl XcafStyleStubOmm {
    /// Style equality used by the styles double-map.
    fn same_style(&self, other: &XcafStyleStubOmm) -> bool {
        let tex_eq = match (&self.base_color_texture, &other.base_color_texture) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        self.material == other.material && self.color_surf_rgba == other.color_surf_rgba && tex_eq
    }
}

/// Material MTL file writer for OBJ export.
pub struct RWObjObjMaterialMap {
    file_name: String,
    key_prefix: String,
    /// DoubleMap<XCAFPrs_Style, AsciiString> modeled as ordered pairs.
    styles: Vec<(XcafStyleStubOmm, String)>,
    /// Images copied so far: (texture handle, relative path in MTL).
    image_map: Vec<(HandleImageTextureOmm, String)>,
    /// Images that failed to copy.
    image_fail_map: Vec<HandleImageTextureOmm>,
    default_style: XcafStyleStubOmm,
    nb_materials: i32,
    is_failed: bool,
    mat_name_as_key: bool,
    /// In-memory MTL destination; None until first AddMaterial (lazy open).
    file: Option<String>,
    closed_content: Option<String>,
}

impl RWObjObjMaterialMap {
    /// Main constructor (file is NOT opened yet — lazily on AddMaterial).
    pub fn new(file: &str) -> Self {
        RWObjObjMaterialMap {
            file_name: file.to_string(),
            key_prefix: "mat_".to_string(),
            styles: Vec::new(),
            image_map: Vec::new(),
            image_fail_map: Vec::new(),
            default_style: XcafStyleStubOmm::default(),
            nb_materials: 0,
            is_failed: false,
            mat_name_as_key: true,
            file: None,
            closed_content: None,
        }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// RWMesh_MaterialMap::IsFailed.
    pub fn is_failed(&self) -> bool {
        self.is_failed
    }

    /// RWMesh_MaterialMap::SetDefaultStyle.
    pub fn set_default_style(&mut self, style: XcafStyleStubOmm) {
        self.default_style = style;
    }

    /// RWMesh_MaterialMap::FindMaterial — empty string when unknown.
    pub fn find_material(&self, style: &XcafStyleStubOmm) -> String {
        self.styles
            .iter()
            .find(|(s, _)| s.same_style(style))
            .map(|(_, k)| k.clone())
            .unwrap_or_default()
    }

    fn is_bound2(&self, key: &str) -> bool {
        self.styles.iter().any(|(_, k)| k == key)
    }

    fn emit(&mut self, s: &str) {
        if let Some(f) = &mut self.file {
            f.push_str(s);
        }
    }

    /// RWObj_ObjMaterialMap::AddMaterial — lazily opens the MTL file and
    /// writes its comment header, then registers via the base logic.
    pub fn add_material(&mut self, style: &XcafStyleStubOmm) -> String {
        if self.file.is_none() && !self.is_failed {
            self.file = Some(String::new());
            self.emit("# Exported by Open CASCADE Technology [dev.opencascade.org]\n");
        }
        if self.file.is_none() {
            return String::new();
        }
        self.add_material_base(style)
    }

    /// RWMesh_MaterialMap::AddMaterial (base registration algorithm).
    fn add_material_base(&mut self, style: &XcafStyleStubOmm) -> String {
        if let Some((_, key)) = self.styles.iter().find(|(s, _)| s.same_style(style)) {
            return key.clone();
        }

        let mut mat_key;
        let mat_name_suffix;
        let mut local_counter = 0i32;
        let use_local_counter;

        // myMatNameAsKey branch (always true for the OBJ writer).
        assert!(self.mat_name_as_key);
        if let Some(mat) = &style.material {
            use_local_counter = true;
            let mat_name = mat.label_name.clone().unwrap_or_else(|| "mat".to_string());
            mat_name_suffix = mat_name.clone();
            mat_key = mat_name;
        } else {
            self.nb_materials += 1;
            use_local_counter = false;
            mat_name_suffix = self.key_prefix.clone();
            mat_key = format!("{}{}", mat_name_suffix, self.nb_materials);
        }

        // Collision resolution loop.
        loop {
            if self.is_bound2(&mat_key) {
                let counter = if use_local_counter {
                    let c = local_counter;
                    local_counter += 1;
                    c
                } else {
                    self.nb_materials += 1;
                    self.nb_materials
                };
                mat_key = format!("{mat_name_suffix}{counter}");
                continue;
            }
            break;
        }

        self.styles.push((style.clone(), mat_key.clone()));
        self.define_material(style, &mat_key, &mat_key);
        mat_key
    }

    /// RWObj_ObjMaterialMap::DefineMaterial — writes the MTL entry.
    pub fn define_material(&mut self, style: &XcafStyleStubOmm, key: &str, _name: &str) {
        self.emit(&format!("newmtl {key}\n"));

        let def_mat = self
            .default_style
            .material
            .clone()
            .unwrap_or_default();
        let mut amb = def_mat.ambient;
        let mut diff = def_mat.diffuse;
        let mut spec = def_mat.specular;
        let mut transp: f32 = 0.0;
        let mut specular_exp: f32 = def_mat.shininess * 1000.0;
        let mut has_material = false;

        if let Some(mat) = &style.material {
            has_material = true;
            amb = mat.ambient;
            diff = mat.diffuse;
            spec = mat.specular;
            transp = mat.transparency;
            specular_exp = mat.shininess * 1000.0;
        }
        if let Some((color, alpha)) = &style.color_surf_rgba {
            has_material = true;
            diff = *color;
            amb = [color[0] * 0.25, color[1] * 0.25, color[2] * 0.25];
            if *alpha < 1.0 {
                transp = 1.0 - *alpha;
            }
        }

        if has_material {
            self.emit(&format!("Ka {:.6} {:.6} {:.6}\n", amb[0], amb[1], amb[2]));
            self.emit(&format!("Kd {:.6} {:.6} {:.6}\n", diff[0], diff[1], diff[2]));
            self.emit(&format!("Ks {:.6} {:.6} {:.6}\n", spec[0], spec[1], spec[2]));
            self.emit(&format!("Ns {:.6}\n", specular_exp));
            if transp >= 0.0001 {
                self.emit(&format!("Tr {:.6}\n", transp));
            }
        }

        if let Some(base_texture) = &style.base_color_texture {
            let already = self
                .image_map
                .iter()
                .find(|(t, _)| Rc::ptr_eq(t, base_texture))
                .map(|(_, p)| p.clone());
            let failed = self.image_fail_map.iter().any(|t| Rc::ptr_eq(t, base_texture));
            let texture_path = if let Some(p) = already {
                p
            } else if !failed {
                match self.copy_texture(base_texture, &format!("{}", self.image_map.len() + 1)) {
                    Some(p) => {
                        self.image_map.push((base_texture.clone(), p.clone()));
                        p
                    }
                    None => {
                        self.image_fail_map.push(base_texture.clone());
                        String::new()
                    }
                }
            } else {
                String::new()
            };
            if !texture_path.is_empty() {
                self.emit(&format!("map_Kd {texture_path}\n"));
            }
        }
    }

    /// RWMesh_MaterialMap::CopyTexture modeled as producing the relative
    /// texture path "textures/<key><ext>"; fails on empty source.
    fn copy_texture(&self, texture: &HandleImageTextureOmm, key: &str) -> Option<String> {
        if texture.source_path.is_empty() {
            return None;
        }
        let ext = texture
            .source_path
            .rfind('.')
            .map(|i| texture.source_path[i..].to_string())
            .unwrap_or_default();
        Some(format!("textures/{key}{ext}"))
    }

    /// Destructor behavior: close the file; failure flag reported.
    pub fn close(&mut self) -> bool {
        match self.file.take() {
            Some(content) => {
                self.closed_content = Some(content);
                true
            }
            None => false,
        }
    }

    /// Written MTL text (for inspection in tests).
    pub fn written_text(&self) -> &str {
        if let Some(f) = &self.file {
            f
        } else if let Some(c) = &self.closed_content {
            c
        } else {
            ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color_style(rgb: [f64; 3], alpha: f32) -> XcafStyleStubOmm {
        XcafStyleStubOmm {
            material: None,
            color_surf_rgba: Some((rgb, alpha)),
            base_color_texture: None,
        }
    }

    #[test]
    fn lazy_open_writes_header_once() {
        let mut map = RWObjObjMaterialMap::new("model.mtl");
        assert_eq!(map.written_text(), "", "file not opened before first material");
        map.add_material(&color_style([1.0, 0.0, 0.0], 1.0));
        map.add_material(&color_style([0.0, 1.0, 0.0], 1.0));
        let text = map.written_text();
        assert_eq!(
            text.matches("# Exported by Open CASCADE Technology").count(),
            1
        );
    }

    #[test]
    fn generated_keys_and_dedup() {
        let mut map = RWObjObjMaterialMap::new("m.mtl");
        let red = color_style([1.0, 0.0, 0.0], 1.0);
        let k1 = map.add_material(&red);
        assert_eq!(k1, "mat_1");
        // Same style registered again -> same key, no new entry.
        let k1b = map.add_material(&red.clone());
        assert_eq!(k1b, "mat_1");
        assert_eq!(map.written_text().matches("newmtl").count(), 1);
        let k2 = map.add_material(&color_style([0.0, 0.0, 1.0], 1.0));
        assert_eq!(k2, "mat_2");
        assert_eq!(map.find_material(&red), "mat_1");
    }

    #[test]
    fn named_material_key_and_collision_suffix() {
        let mut map = RWObjObjMaterialMap::new("m.mtl");
        let steel_a = XcafStyleStubOmm {
            material: Some(VisMaterialCommonOmm {
                label_name: Some("steel".into()),
                ..VisMaterialCommonOmm::default()
            }),
            ..XcafStyleStubOmm::default()
        };
        let mut steel_b = steel_a.clone();
        steel_b.material.as_mut().unwrap().shininess = 0.5; // different style, same name
        assert_eq!(map.add_material(&steel_a), "steel");
        assert_eq!(map.add_material(&steel_b), "steel0", "name collision suffixed");
    }

    #[test]
    fn mtl_statements_for_colored_style() {
        let mut map = RWObjObjMaterialMap::new("m.mtl");
        map.add_material(&color_style([1.0, 0.5, 0.0], 0.75));
        let text = map.written_text();
        assert!(text.contains("newmtl mat_1\n"));
        assert!(text.contains("Kd 1.000000 0.500000 0.000000\n"));
        // Ambient = diffuse * 0.25.
        assert!(text.contains("Ka 0.250000 0.125000 0.000000\n"));
        // Transparency = 1 - alpha.
        assert!(text.contains("Tr 0.250000\n"));
        // Default specular/shininess from default style material defaults.
        assert!(text.contains("Ks 0.200000 0.200000 0.200000\n"));
        assert!(text.contains("Ns 1000.000000\n"));
    }

    #[test]
    fn opaque_color_has_no_tr_statement() {
        let mut map = RWObjObjMaterialMap::new("m.mtl");
        map.add_material(&color_style([0.2, 0.2, 0.2], 1.0));
        assert!(!map.written_text().contains("Tr "));
    }

    #[test]
    fn texture_copied_once_and_mapped() {
        let mut map = RWObjObjMaterialMap::new("m.mtl");
        let tex = Rc::new(ImageTextureStubOmm { source_path: "wood.png".into() });
        let style1 = XcafStyleStubOmm {
            color_surf_rgba: Some(([1.0, 1.0, 1.0], 1.0)),
            base_color_texture: Some(tex.clone()),
            ..XcafStyleStubOmm::default()
        };
        let mut style2 = style1.clone();
        style2.color_surf_rgba = Some(([0.5, 0.5, 0.5], 1.0));
        style2.base_color_texture = Some(tex.clone());
        map.add_material(&style1);
        map.add_material(&style2);
        let text = map.written_text();
        assert_eq!(text.matches("map_Kd textures/1.png\n").count(), 2, "same copied file reused");
        // Failing texture (empty source) emits no map_Kd.
        let bad = XcafStyleStubOmm {
            base_color_texture: Some(Rc::new(ImageTextureStubOmm { source_path: String::new() })),
            ..XcafStyleStubOmm::default()
        };
        map.add_material(&bad);
        assert_eq!(map.written_text().matches("map_Kd").count(), 2);
    }

    #[test]
    fn close_semantics() {
        let mut map = RWObjObjMaterialMap::new("m.mtl");
        assert!(!map.close(), "nothing to close before lazy open");
        map.add_material(&color_style([0.0, 0.0, 0.0], 1.0));
        assert!(map.close());
        assert!(map.written_text().contains("newmtl"));
    }
}
