// FILE: bin_mxcaf_doc_vis_material_driver.rs
// occt: BinMXCAFDoc_VisMaterialDriver
//
// Faithful port of OCCT BinMXCAFDoc_VisMaterialDriver
// (BinMXCAFDoc_VisMaterialDriver.cxx). Payload layout per the .cxx:
//   <Byte verMajor> <Byte verMinor>
//   <Byte faceCullChar> <Byte alphaModeChar> <ShortReal alphaCutOff>
//   <Boolean hasPbr>
//   [ <vec4 baseColor> <vec3 emissiveFactor> <ShortReal metallic>
//     <ShortReal roughness> <texture baseColor> <texture metallicRoughness>
//     <texture emissive> <texture occlusion> <texture normal> ]
//   <Boolean hasCommon>
//   [ <vec3 ambient> <vec3 diffuse> <vec3 specular> <vec3 emissive>
//     <ShortReal shininess> <ShortReal transparency> <texture diffuse> ]
//   [ <ShortReal refractionIndex>   when hasPbr and version >= 1.1 ]
// Texture encoding follows writeTexture()/readTexture() exactly.
//
// BinObjMgt_Persistent is modelled locally by VismatPersistentStream
// (big-endian file byte order, 4-byte word alignment for integers, short
// reals and string starts; bytes and byte arrays unaligned).

use std::cell::RefCell;

/// Version constants from BinMXCAFDoc_VisMaterialDriver.hxx.
pub const VISMAT_VERSION_MAJOR_1: u8 = 1;
pub const VISMAT_VERSION_MINOR_0: u8 = 0;
pub const VISMAT_VERSION_MINOR_1: u8 = 1; // added IOR
pub const VISMAT_VERSION_MAJOR: u8 = VISMAT_VERSION_MAJOR_1;
pub const VISMAT_VERSION_MINOR: u8 = VISMAT_VERSION_MINOR_1;

/// Graphic3d_AlphaMode, as encoded by alphaModeToChar/alphaModeFromChar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VismatAlphaMode {
    Opaque,
    Mask,
    Blend,
    MaskBlend,
    BlendAuto,
}

/// Mirrors static alphaModeToChar().
pub fn vismat_alpha_mode_to_char(mode: VismatAlphaMode) -> u8 {
    match mode {
        VismatAlphaMode::Opaque => b'O',
        VismatAlphaMode::Mask => b'M',
        VismatAlphaMode::Blend => b'B',
        VismatAlphaMode::MaskBlend => b'b',
        VismatAlphaMode::BlendAuto => b'A',
    }
}

/// Mirrors static alphaModeFromChar().
pub fn vismat_alpha_mode_from_char(c: u8) -> VismatAlphaMode {
    match c {
        b'O' => VismatAlphaMode::Opaque,
        b'M' => VismatAlphaMode::Mask,
        b'B' => VismatAlphaMode::Blend,
        b'b' => VismatAlphaMode::MaskBlend,
        b'A' => VismatAlphaMode::BlendAuto,
        _ => VismatAlphaMode::BlendAuto,
    }
}

/// Graphic3d_TypeOfBackfacingModel, encoded by faceCullToChar/faceCullFromChar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VismatFaceCull {
    Auto,
    BackCulled,
    FrontCulled,
    DoubleSided,
}

/// Mirrors static faceCullToChar().
pub fn vismat_face_cull_to_char(mode: VismatFaceCull) -> u8 {
    match mode {
        VismatFaceCull::Auto => b'0',
        VismatFaceCull::BackCulled => b'B',
        VismatFaceCull::FrontCulled => b'F',
        VismatFaceCull::DoubleSided => b'1',
    }
}

/// Mirrors static faceCullFromChar().
pub fn vismat_face_cull_from_char(c: u8) -> VismatFaceCull {
    match c {
        b'0' => VismatFaceCull::Auto,
        b'B' => VismatFaceCull::BackCulled,
        b'F' => VismatFaceCull::FrontCulled,
        b'1' => VismatFaceCull::DoubleSided,
        _ => VismatFaceCull::Auto,
    }
}

/// Local model of Image_Texture in the three persistable states handled by
/// writeTexture()/readTexture().
#[derive(Debug, Clone, PartialEq)]
pub enum VismatTexture {
    /// File path only (FileOffset()/FileLength() == -1).
    FilePath(String),
    /// File path with offset/length region.
    FileRegion { path: String, offset: i32, length: i32 },
    /// In-memory buffer with its generated texture id.
    Buffer { texture_id: String, data: Vec<u8> },
}

/// Local model of XCAFDoc_VisMaterialPBR.
#[derive(Debug, Clone, PartialEq)]
pub struct VismatPbrMaterial {
    pub base_color: [f32; 4],
    pub emissive_factor: [f32; 3],
    pub metallic: f32,
    pub roughness: f32,
    pub refraction_index: f32,
    pub base_color_texture: Option<VismatTexture>,
    pub metallic_roughness_texture: Option<VismatTexture>,
    pub emissive_texture: Option<VismatTexture>,
    pub occlusion_texture: Option<VismatTexture>,
    pub normal_texture: Option<VismatTexture>,
}

impl Default for VismatPbrMaterial {
    fn default() -> Self {
        // Defaults of XCAFDoc_VisMaterialPBR.
        VismatPbrMaterial {
            base_color: [1.0, 1.0, 1.0, 1.0],
            emissive_factor: [0.0, 0.0, 0.0],
            metallic: 1.0,
            roughness: 1.0,
            refraction_index: 1.5,
            base_color_texture: None,
            metallic_roughness_texture: None,
            emissive_texture: None,
            occlusion_texture: None,
            normal_texture: None,
        }
    }
}

/// Local model of XCAFDoc_VisMaterialCommon.
#[derive(Debug, Clone, PartialEq)]
pub struct VismatCommonMaterial {
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub emissive_color: [f32; 3],
    pub shininess: f32,
    pub transparency: f32,
    pub diffuse_texture: Option<VismatTexture>,
}

impl Default for VismatCommonMaterial {
    fn default() -> Self {
        VismatCommonMaterial {
            ambient_color: [0.1, 0.1, 0.1],
            diffuse_color: [0.8, 0.8, 0.8],
            specular_color: [0.2, 0.2, 0.2],
            emissive_color: [0.0, 0.0, 0.0],
            shininess: 1.0,
            transparency: 0.0,
            diffuse_texture: None,
        }
    }
}

/// Local model of the XCAFDoc_VisMaterial attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct XcafVisMaterialAttribute {
    face_culling: VismatFaceCull,
    alpha_mode: VismatAlphaMode,
    alpha_cutoff: f32,
    pbr: Option<VismatPbrMaterial>,
    common: Option<VismatCommonMaterial>,
}

impl XcafVisMaterialAttribute {
    /// Mirrors `new XCAFDoc_VisMaterial()`.
    pub fn new_empty() -> Self {
        XcafVisMaterialAttribute {
            face_culling: VismatFaceCull::Auto,
            alpha_mode: VismatAlphaMode::BlendAuto,
            alpha_cutoff: 0.5,
            pbr: None,
            common: None,
        }
    }

    pub fn set_face_culling(&mut self, mode: VismatFaceCull) {
        self.face_culling = mode;
    }

    pub fn face_culling(&self) -> VismatFaceCull {
        self.face_culling
    }

    pub fn set_alpha_mode(&mut self, mode: VismatAlphaMode, cutoff: f32) {
        self.alpha_mode = mode;
        self.alpha_cutoff = cutoff;
    }

    pub fn alpha_mode(&self) -> VismatAlphaMode {
        self.alpha_mode
    }

    pub fn alpha_cutoff(&self) -> f32 {
        self.alpha_cutoff
    }

    pub fn set_pbr_material(&mut self, pbr: VismatPbrMaterial) {
        self.pbr = Some(pbr);
    }

    pub fn has_pbr_material(&self) -> bool {
        self.pbr.is_some()
    }

    pub fn pbr_material(&self) -> Option<&VismatPbrMaterial> {
        self.pbr.as_ref()
    }

    pub fn set_common_material(&mut self, common: VismatCommonMaterial) {
        self.common = Some(common);
    }

    pub fn has_common_material(&self) -> bool {
        self.common.is_some()
    }

    pub fn common_material(&self) -> Option<&VismatCommonMaterial> {
        self.common.as_ref()
    }
}

/// Local stand-in for BinObjMgt_Persistent.
pub struct VismatPersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl VismatPersistentStream {
    pub fn new() -> Self {
        VismatPersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        VismatPersistentStream {
            data: bytes.to_vec(),
            pos: 0,
            err: false,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn is_error(&self) -> bool {
        self.err
    }

    fn align_put(&mut self, n: usize) {
        while self.data.len() % n != 0 {
            self.data.push(0);
        }
    }

    fn align_get(&mut self, n: usize) {
        while self.pos % n != 0 {
            self.pos += 1;
        }
    }

    /// BinObjMgt_Persistent::PutByte (unaligned).
    pub fn put_byte(&mut self, v: u8) {
        self.data.push(v);
    }

    /// BinObjMgt_Persistent::GetByte.
    pub fn get_byte(&mut self) -> Option<u8> {
        if self.pos >= self.data.len() {
            self.err = true;
            return None;
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Some(v)
    }

    /// BinObjMgt_Persistent::PutInteger.
    pub fn put_integer(&mut self, v: i32) {
        self.align_put(4);
        self.data.extend_from_slice(&v.to_be_bytes());
    }

    /// BinObjMgt_Persistent::GetInteger.
    pub fn get_integer(&mut self) -> Option<i32> {
        self.align_get(4);
        if self.pos + 4 > self.data.len() {
            self.err = true;
            return None;
        }
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Some(i32::from_be_bytes(b))
    }

    /// BinObjMgt_Persistent::PutBoolean — stored as PutInteger(0/1).
    pub fn put_boolean(&mut self, v: bool) {
        self.put_integer(if v { 1 } else { 0 });
    }

    /// BinObjMgt_Persistent::GetBoolean.
    pub fn get_boolean(&mut self) -> Option<bool> {
        self.get_integer().map(|v| v != 0)
    }

    /// BinObjMgt_Persistent::PutShortReal — 4-byte aligned float.
    pub fn put_short_real(&mut self, v: f32) {
        self.align_put(4);
        self.data.extend_from_slice(&v.to_bits().to_be_bytes());
    }

    /// BinObjMgt_Persistent::GetShortReal.
    pub fn get_short_real(&mut self) -> Option<f32> {
        self.align_get(4);
        if self.pos + 4 > self.data.len() {
            self.err = true;
            return None;
        }
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Some(f32::from_bits(u32::from_be_bytes(b)))
    }

    /// BinObjMgt_Persistent::PutAsciiString — word-aligned, NUL-terminated.
    pub fn put_ascii_string(&mut self, s: &str) {
        self.align_put(4);
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0);
    }

    /// BinObjMgt_Persistent::GetAsciiString.
    pub fn get_ascii_string(&mut self) -> Option<String> {
        self.align_get(4);
        let start = self.pos;
        while self.pos < self.data.len() && self.data[self.pos] != 0 {
            self.pos += 1;
        }
        if self.pos >= self.data.len() {
            self.err = true;
            self.pos = start;
            return None;
        }
        let s = String::from_utf8_lossy(&self.data[start..self.pos]).into_owned();
        self.pos += 1;
        Some(s)
    }

    /// BinObjMgt_Persistent::PutByteArray (unaligned raw bytes).
    pub fn put_byte_array(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    /// BinObjMgt_Persistent::GetByteArray.
    pub fn get_byte_array(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.pos + len > self.data.len() {
            self.err = true;
            return None;
        }
        let out = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Some(out)
    }
}

impl Default for VismatPersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_VisMaterialDriver.
pub struct BinMXCAFDocVisMaterialDriver {
    name: String,
    /// Local model of myMessageDriver->Send().
    messages: RefCell<Vec<String>>,
}

impl BinMXCAFDocVisMaterialDriver {
    pub fn new() -> Self {
        BinMXCAFDocVisMaterialDriver {
            name: "XCAFDoc_VisMaterial".to_string(),
            messages: RefCell::new(Vec::new()),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    pub fn new_empty(&self) -> XcafVisMaterialAttribute {
        XcafVisMaterialAttribute::new_empty()
    }

    pub fn messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }

    /// Mirrors static writeVec3().
    fn write_vec3(target: &mut VismatPersistentStream, v: &[f32; 3]) {
        target.put_short_real(v[0]);
        target.put_short_real(v[1]);
        target.put_short_real(v[2]);
    }

    /// Mirrors static writeVec4().
    fn write_vec4(target: &mut VismatPersistentStream, v: &[f32; 4]) {
        target.put_short_real(v[0]);
        target.put_short_real(v[1]);
        target.put_short_real(v[2]);
        target.put_short_real(v[3]);
    }

    /// Mirrors static readVec3().
    fn read_vec3(source: &mut VismatPersistentStream) -> Option<[f32; 3]> {
        Some([
            source.get_short_real()?,
            source.get_short_real()?,
            source.get_short_real()?,
        ])
    }

    /// Mirrors static readColor(Quantity_ColorRGBA&).
    fn read_vec4(source: &mut VismatPersistentStream) -> Option<[f32; 4]> {
        Some([
            source.get_short_real()?,
            source.get_short_real()?,
            source.get_short_real()?,
            source.get_short_real()?,
        ])
    }

    /// Mirrors static writeTexture().
    fn write_texture(target: &mut VismatPersistentStream, tex: &Option<VismatTexture>) {
        match tex {
            None => {
                target.put_ascii_string("");
            }
            Some(VismatTexture::FilePath(path)) => {
                target.put_ascii_string(path);
                target.put_boolean(false);
                // FileOffset()/FileLength() == -1 => "only file path" flag
                target.put_boolean(true);
            }
            Some(VismatTexture::FileRegion {
                path,
                offset,
                length,
            }) => {
                target.put_ascii_string(path);
                target.put_boolean(false);
                target.put_boolean(false);
                target.put_integer(*offset);
                target.put_integer(*length);
            }
            Some(VismatTexture::Buffer { texture_id, data }) => {
                target.put_ascii_string(texture_id);
                target.put_boolean(true);
                target.put_integer(data.len() as i32);
                target.put_byte_array(data);
            }
        }
    }

    /// Mirrors static readTexture().
    fn read_texture(source: &mut VismatPersistentStream) -> Option<VismatTexture> {
        let a_str = match source.get_ascii_string() {
            Some(s) => s,
            None => return None,
        };
        if a_str.is_empty() {
            return None;
        }
        let an_use_buffer = match source.get_boolean() {
            Some(b) => b,
            // `if (!theSource.GetBoolean(anUseBuffer).IsOK())` -> path only
            None => return Some(VismatTexture::FilePath(a_str)),
        };
        if !an_use_buffer {
            let is_only_file_path = source.get_boolean().unwrap_or(true);
            if is_only_file_path {
                return Some(VismatTexture::FilePath(a_str));
            }
            let offset = source.get_integer().unwrap_or(-1);
            let length = source.get_integer().unwrap_or(-1);
            return Some(VismatTexture::FileRegion {
                path: a_str,
                offset,
                length,
            });
        }
        let length = source.get_integer().unwrap_or(0);
        let data = source.get_byte_array(length.max(0) as usize).unwrap_or_default();
        Some(VismatTexture::Buffer {
            texture_id: a_str,
            data,
        })
    }

    /// Mirrors Paste(read).
    pub fn paste_read(
        &self,
        source: &mut VismatPersistentStream,
        target: &mut XcafVisMaterialAttribute,
    ) -> bool {
        let a_ver_maj = source.get_byte().unwrap_or(0);
        let a_ver_min = source.get_byte().unwrap_or(0);
        if a_ver_maj < 1 || a_ver_maj > VISMAT_VERSION_MAJOR {
            self.messages.borrow_mut().push(format!(
                "Skipping XCAFDoc_VisMaterial of unknown version {}.{} (supported version: {}.{})",
                a_ver_maj, a_ver_min, VISMAT_VERSION_MAJOR, VISMAT_VERSION_MINOR
            ));
            return false;
        }

        let is_double_sided = source.get_byte().unwrap_or(0);
        let an_alpha_mode = source.get_byte().unwrap_or(0);
        let an_alpha_cutoff = source.get_short_real().unwrap_or(0.5);
        target.set_face_culling(vismat_face_cull_from_char(is_double_sided));
        target.set_alpha_mode(vismat_alpha_mode_from_char(an_alpha_mode), an_alpha_cutoff);

        let mut a_pbr_defined = false;
        let mut a_pbr_mat = VismatPbrMaterial::default();
        if let Some(defined) = source.get_boolean() {
            a_pbr_defined = defined;
        }
        if a_pbr_defined {
            a_pbr_mat.base_color = match Self::read_vec4(source) {
                Some(v) => v,
                None => return false,
            };
            a_pbr_mat.emissive_factor = match Self::read_vec3(source) {
                Some(v) => v,
                None => return false,
            };
            a_pbr_mat.metallic = match source.get_short_real() {
                Some(v) => v,
                None => return false,
            };
            a_pbr_mat.roughness = match source.get_short_real() {
                Some(v) => v,
                None => return false,
            };
            a_pbr_mat.base_color_texture = Self::read_texture(source);
            a_pbr_mat.metallic_roughness_texture = Self::read_texture(source);
            a_pbr_mat.emissive_texture = Self::read_texture(source);
            a_pbr_mat.occlusion_texture = Self::read_texture(source);
            a_pbr_mat.normal_texture = Self::read_texture(source);
            target.set_pbr_material(a_pbr_mat.clone());
        }

        let has_com_mat = source.get_boolean().unwrap_or(false);
        if has_com_mat {
            let mut a_com_mat = VismatCommonMaterial::default();
            a_com_mat.ambient_color = match Self::read_vec3(source) {
                Some(v) => v,
                None => return false,
            };
            a_com_mat.diffuse_color = match Self::read_vec3(source) {
                Some(v) => v,
                None => return false,
            };
            a_com_mat.specular_color = match Self::read_vec3(source) {
                Some(v) => v,
                None => return false,
            };
            a_com_mat.emissive_color = match Self::read_vec3(source) {
                Some(v) => v,
                None => return false,
            };
            a_com_mat.shininess = match source.get_short_real() {
                Some(v) => v,
                None => return false,
            };
            a_com_mat.transparency = match source.get_short_real() {
                Some(v) => v,
                None => return false,
            };
            a_com_mat.diffuse_texture = Self::read_texture(source);
            target.set_common_material(a_com_mat);
        }

        // RefractionIndex added in version 1.1.
        if a_ver_maj > VISMAT_VERSION_MAJOR_1
            || (a_ver_maj == VISMAT_VERSION_MAJOR_1 && a_ver_min >= VISMAT_VERSION_MINOR_1)
        {
            if a_pbr_defined {
                if let Some(ior) = source.get_short_real() {
                    a_pbr_mat.refraction_index = ior;
                }
            }
        }

        if a_pbr_defined {
            target.set_pbr_material(a_pbr_mat);
        }

        true
    }

    /// Mirrors Paste(write).
    pub fn paste_write(
        &self,
        source: &XcafVisMaterialAttribute,
        target: &mut VismatPersistentStream,
    ) {
        target.put_byte(VISMAT_VERSION_MAJOR);
        target.put_byte(VISMAT_VERSION_MINOR);

        target.put_byte(vismat_face_cull_to_char(source.face_culling()));
        target.put_byte(vismat_alpha_mode_to_char(source.alpha_mode()));
        target.put_short_real(source.alpha_cutoff());

        target.put_boolean(source.has_pbr_material());
        if let Some(pbr) = source.pbr_material() {
            Self::write_vec4(target, &pbr.base_color);
            Self::write_vec3(target, &pbr.emissive_factor);
            target.put_short_real(pbr.metallic);
            target.put_short_real(pbr.roughness);
            Self::write_texture(target, &pbr.base_color_texture);
            Self::write_texture(target, &pbr.metallic_roughness_texture);
            Self::write_texture(target, &pbr.emissive_texture);
            Self::write_texture(target, &pbr.occlusion_texture);
            Self::write_texture(target, &pbr.normal_texture);
        }

        target.put_boolean(source.has_common_material());
        if let Some(com) = source.common_material() {
            Self::write_vec3(target, &com.ambient_color);
            Self::write_vec3(target, &com.diffuse_color);
            Self::write_vec3(target, &com.specular_color);
            Self::write_vec3(target, &com.emissive_color);
            target.put_short_real(com.shininess);
            target.put_short_real(com.transparency);
            Self::write_texture(target, &com.diffuse_texture);
        }

        if let Some(pbr) = source.pbr_material() {
            target.put_short_real(pbr.refraction_index);
        }
    }
}

impl Default for BinMXCAFDocVisMaterialDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_material() -> XcafVisMaterialAttribute {
        let mut mat = XcafVisMaterialAttribute::new_empty();
        mat.set_face_culling(VismatFaceCull::DoubleSided);
        mat.set_alpha_mode(VismatAlphaMode::Mask, 0.75);
        mat.set_pbr_material(VismatPbrMaterial {
            base_color: [0.9, 0.1, 0.2, 1.0],
            emissive_factor: [0.0, 0.5, 0.0],
            metallic: 0.25,
            roughness: 0.6,
            refraction_index: 1.33,
            base_color_texture: Some(VismatTexture::FilePath("albedo.png".to_string())),
            metallic_roughness_texture: Some(VismatTexture::FileRegion {
                path: "pack.glb".to_string(),
                offset: 1024,
                length: 4096,
            }),
            emissive_texture: Some(VismatTexture::Buffer {
                texture_id: "tex_7".to_string(),
                data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0x42],
            }),
            occlusion_texture: None,
            normal_texture: None,
        });
        mat.set_common_material(VismatCommonMaterial {
            ambient_color: [0.1, 0.2, 0.3],
            diffuse_color: [0.4, 0.5, 0.6],
            specular_color: [0.7, 0.8, 0.9],
            emissive_color: [0.05, 0.0, 0.0],
            shininess: 0.5,
            transparency: 0.25,
            diffuse_texture: Some(VismatTexture::FilePath("diffuse.jpg".to_string())),
        });
        mat
    }

    #[test]
    fn roundtrip_full_material() {
        let driver = BinMXCAFDocVisMaterialDriver::new();
        let src = full_material();

        let mut stream = VismatPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        // Version header comes first.
        assert_eq!(stream.bytes()[0], VISMAT_VERSION_MAJOR);
        assert_eq!(stream.bytes()[1], VISMAT_VERSION_MINOR);

        let mut back = VismatPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst, src);
        // Refraction index survives (version 1.1 field).
        assert_eq!(dst.pbr_material().unwrap().refraction_index, 1.33);
    }

    #[test]
    fn roundtrip_empty_material() {
        let driver = BinMXCAFDocVisMaterialDriver::new();
        let src = driver.new_empty();

        let mut stream = VismatPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = VismatPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert!(!dst.has_pbr_material());
        assert!(!dst.has_common_material());
        assert_eq!(dst.alpha_mode(), VismatAlphaMode::BlendAuto);
        assert_eq!(dst.face_culling(), VismatFaceCull::Auto);
        assert_eq!(dst.alpha_cutoff(), 0.5);
    }

    #[test]
    fn roundtrip_pbr_only_with_buffer_texture() {
        let driver = BinMXCAFDocVisMaterialDriver::new();
        let mut src = driver.new_empty();
        let mut pbr = VismatPbrMaterial::default();
        pbr.normal_texture = Some(VismatTexture::Buffer {
            texture_id: "normals".to_string(),
            data: (0..64u8).collect(),
        });
        pbr.refraction_index = 2.4; // diamond
        src.set_pbr_material(pbr.clone());

        let mut stream = VismatPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = VismatPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.pbr_material().unwrap(), &pbr);
        assert!(!dst.has_common_material());
    }

    #[test]
    fn unknown_version_is_rejected_with_message() {
        let driver = BinMXCAFDocVisMaterialDriver::new();
        let src = driver.new_empty();
        let mut stream = VismatPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        // Corrupt the major version byte to 9.
        let mut bytes = stream.bytes().to_vec();
        bytes[0] = 9;
        let mut back = VismatPersistentStream::from_bytes(&bytes);
        let mut dst = driver.new_empty();
        assert!(!driver.paste_read(&mut back, &mut dst));
        let msgs = driver.messages();
        assert_eq!(msgs.len(), 1);
        assert!(msgs[0].contains("unknown version 9"));
    }

    #[test]
    fn alpha_and_cull_char_codes_match_occt() {
        assert_eq!(vismat_alpha_mode_to_char(VismatAlphaMode::Opaque), b'O');
        assert_eq!(vismat_alpha_mode_to_char(VismatAlphaMode::Mask), b'M');
        assert_eq!(vismat_alpha_mode_to_char(VismatAlphaMode::Blend), b'B');
        assert_eq!(vismat_alpha_mode_to_char(VismatAlphaMode::MaskBlend), b'b');
        assert_eq!(vismat_alpha_mode_to_char(VismatAlphaMode::BlendAuto), b'A');
        assert_eq!(vismat_face_cull_to_char(VismatFaceCull::Auto), b'0');
        assert_eq!(vismat_face_cull_to_char(VismatFaceCull::BackCulled), b'B');
        assert_eq!(vismat_face_cull_to_char(VismatFaceCull::FrontCulled), b'F');
        assert_eq!(vismat_face_cull_to_char(VismatFaceCull::DoubleSided), b'1');
        // from_char is the inverse, with fallback defaults
        for m in [
            VismatAlphaMode::Opaque,
            VismatAlphaMode::Mask,
            VismatAlphaMode::Blend,
            VismatAlphaMode::MaskBlend,
            VismatAlphaMode::BlendAuto,
        ] {
            assert_eq!(vismat_alpha_mode_from_char(vismat_alpha_mode_to_char(m)), m);
        }
        assert_eq!(vismat_alpha_mode_from_char(b'?'), VismatAlphaMode::BlendAuto);
        assert_eq!(vismat_face_cull_from_char(b'?'), VismatFaceCull::Auto);
    }
}
