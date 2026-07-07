// FILE: xml_mxcaf_doc_vis_material_driver.rs
// occt: XmlMXCAFDoc_VisMaterialDriver
//
// Faithful port of OCCT XmlMXCAFDoc_VisMaterialDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_VisMaterialDriver.hxx),
// the XmlMDF_ADriver for XCAF visual material attributes.
// Serializes/deserializes XCAFDoc_VisMaterial data (surface finish, color,
// reflectance model: Metallic/Specular/Matte).

/// Local model of visual material surface type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SurfaceType {
    Metallic,
    Specular,
    Matte,
}

impl SurfaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SurfaceType::Metallic => "Metallic",
            SurfaceType::Specular => "Specular",
            SurfaceType::Matte => "Matte",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Metallic" => Some(SurfaceType::Metallic),
            "Specular" => Some(SurfaceType::Specular),
            "Matte" => Some(SurfaceType::Matte),
            _ => None,
        }
    }
}

/// Local model of visual material RGBA color.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct VisMaterialColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl VisMaterialColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

/// Local model of visual material data.
#[derive(Debug, Clone, PartialEq)]
pub struct VisMaterialData {
    pub surface_type: SurfaceType,
    pub color: VisMaterialColor,
    pub shininess: f64,
}

impl VisMaterialData {
    pub fn new(surface_type: SurfaceType, color: VisMaterialColor, shininess: f64) -> Self {
        Self {
            surface_type,
            color,
            shininess,
        }
    }

    pub fn default_material() -> Self {
        Self {
            surface_type: SurfaceType::Matte,
            color: VisMaterialColor::white(),
            shininess: 0.5,
        }
    }
}

/// XmlMDF_ADriver for visual material attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocVisMaterialDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocVisMaterialDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_VisMaterial";

    pub fn new() -> Self {
        Self {
            type_name: Self::TYPE_NAME.to_string(),
            version: 1,
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn version_number(&self) -> u32 {
        self.version
    }

    /// Read visual material from XML element text.
    /// Format: "surface_type r g b a shininess"
    /// (11 space-separated values: surface type, 4 color channels, shininess).
    pub fn read_from_xml(&self, element_text: &str) -> Result<VisMaterialData, String> {
        let mut parts = element_text.split_whitespace();
        let surface_str = parts
            .next()
            .ok_or_else(|| "Missing surface type".to_string())?;
        let r_str = parts
            .next()
            .ok_or_else(|| "Missing red channel".to_string())?;
        let g_str = parts
            .next()
            .ok_or_else(|| "Missing green channel".to_string())?;
        let b_str = parts
            .next()
            .ok_or_else(|| "Missing blue channel".to_string())?;
        let a_str = parts
            .next()
            .ok_or_else(|| "Missing alpha channel".to_string())?;
        let shininess_str = parts
            .next()
            .ok_or_else(|| "Missing shininess".to_string())?;

        let surface_type = SurfaceType::from_str(surface_str)
            .ok_or_else(|| format!("Unknown surface type: {}", surface_str))?;

        let r = r_str
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse red: {}", e))?;
        let g = g_str
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse green: {}", e))?;
        let b = b_str
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse blue: {}", e))?;
        let a = a_str
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse alpha: {}", e))?;
        let shininess = shininess_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse shininess: {}", e))?;

        Ok(VisMaterialData {
            surface_type,
            color: VisMaterialColor::new(r, g, b, a),
            shininess,
        })
    }

    /// Write visual material to XML element text.
    pub fn write_to_xml(&self, data: &VisMaterialData) -> String {
        format!(
            "{} {} {} {} {} {}",
            data.surface_type.as_str(),
            data.color.r,
            data.color.g,
            data.color.b,
            data.color.a,
            data.shininess
        )
    }
}

impl Default for XmlMXCAFDocVisMaterialDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_type_as_str() {
        assert_eq!(SurfaceType::Metallic.as_str(), "Metallic");
        assert_eq!(SurfaceType::Specular.as_str(), "Specular");
        assert_eq!(SurfaceType::Matte.as_str(), "Matte");
    }

    #[test]
    fn test_surface_type_from_str() {
        assert_eq!(SurfaceType::from_str("Metallic"), Some(SurfaceType::Metallic));
        assert_eq!(SurfaceType::from_str("Specular"), Some(SurfaceType::Specular));
        assert_eq!(SurfaceType::from_str("Matte"), Some(SurfaceType::Matte));
        assert_eq!(SurfaceType::from_str("Unknown"), None);
    }

    #[test]
    fn test_vis_material_color_new() {
        let color = VisMaterialColor::new(100, 150, 200, 255);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_vis_material_color_white() {
        let color = VisMaterialColor::white();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_vis_material_data_new() {
        let color = VisMaterialColor::new(200, 100, 50, 255);
        let material = VisMaterialData::new(SurfaceType::Metallic, color, 0.8);
        assert_eq!(material.surface_type, SurfaceType::Metallic);
        assert!((material.shininess - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_vis_material_data_default() {
        let material = VisMaterialData::default_material();
        assert_eq!(material.surface_type, SurfaceType::Matte);
        assert_eq!(material.color, VisMaterialColor::white());
        assert!((material.shininess - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocVisMaterialDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_VisMaterial");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_metallic() {
        let driver = XmlMXCAFDocVisMaterialDriver::new();
        let result = driver.read_from_xml("Metallic 200 200 200 255 0.9");
        assert!(result.is_ok());
        let material = result.unwrap();
        assert_eq!(material.surface_type, SurfaceType::Metallic);
        assert_eq!(material.color.r, 200);
        assert!((material.shininess - 0.9).abs() < 1e-10);
    }

    #[test]
    fn test_read_from_xml_matte() {
        let driver = XmlMXCAFDocVisMaterialDriver::new();
        let result = driver.read_from_xml("Matte 100 100 100 255 0.3");
        assert!(result.is_ok());
        let material = result.unwrap();
        assert_eq!(material.surface_type, SurfaceType::Matte);
    }

    #[test]
    fn test_read_from_xml_invalid_surface_type() {
        let driver = XmlMXCAFDocVisMaterialDriver::new();
        let result = driver.read_from_xml("InvalidType 255 255 255 255 0.5");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_invalid_color_value() {
        let driver = XmlMXCAFDocVisMaterialDriver::new();
        let result = driver.read_from_xml("Specular 256 255 255 255 0.5");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocVisMaterialDriver::new();
        let color = VisMaterialColor::new(150, 100, 50, 255);
        let material = VisMaterialData::new(SurfaceType::Specular, color, 0.7);
        let xml = driver.write_to_xml(&material);
        assert_eq!(xml, "Specular 150 100 50 255 0.7");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocVisMaterialDriver::new();
        let color = VisMaterialColor::new(180, 140, 100, 255);
        let original = VisMaterialData::new(SurfaceType::Metallic, color, 0.85);
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original.surface_type, restored.surface_type);
        assert_eq!(original.color, restored.color);
        assert!((original.shininess - restored.shininess).abs() < 1e-10);
    }
}
