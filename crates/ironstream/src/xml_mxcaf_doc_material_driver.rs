// FILE: xml_mxcaf_doc_material_driver.rs
// occt: XmlMXCAFDoc_MaterialDriver
//
// Faithful port of OCCT XmlMXCAFDoc_MaterialDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_MaterialDriver.hxx),
// the XmlMDF_ADriver for XCAF material attributes.
// Serializes/deserializes XCAFDoc_Material data (material reference: density, name, etc.).

/// Local model of material data.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialData {
    pub name: String,
    pub density: f64,
    pub description: String,
}

impl MaterialData {
    pub fn new(name: &str, density: f64, description: &str) -> Self {
        Self {
            name: name.to_string(),
            density,
            description: description.to_string(),
        }
    }

    pub fn default_material() -> Self {
        Self {
            name: "Default".to_string(),
            density: 1.0,
            description: String::new(),
        }
    }
}

/// XmlMDF_ADriver for material attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocMaterialDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocMaterialDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_Material";

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

    /// Read material from XML element text.
    /// Format: "name density description" or "name density" if no description.
    /// Density is a floating-point number; name and description are identifiers/strings.
    pub fn read_from_xml(&self, element_text: &str) -> Result<MaterialData, String> {
        let mut parts = element_text.split_whitespace();
        let name = parts
            .next()
            .ok_or_else(|| "Missing material name".to_string())?
            .to_string();
        let density_str = parts
            .next()
            .ok_or_else(|| "Missing material density".to_string())?;

        let density = density_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse density: {}", e))?;

        let description = parts.collect::<Vec<_>>().join(" ");

        Ok(MaterialData {
            name,
            density,
            description,
        })
    }

    /// Write material to XML element text.
    pub fn write_to_xml(&self, data: &MaterialData) -> String {
        if data.description.is_empty() {
            format!("{} {}", data.name, data.density)
        } else {
            format!("{} {} {}", data.name, data.density, data.description)
        }
    }
}

impl Default for XmlMXCAFDocMaterialDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_data_new() {
        let mat = MaterialData::new("Steel", 7.85, "Carbon Steel");
        assert_eq!(mat.name, "Steel");
        assert!((mat.density - 7.85).abs() < 1e-10);
        assert_eq!(mat.description, "Carbon Steel");
    }

    #[test]
    fn test_material_data_default() {
        let mat = MaterialData::default_material();
        assert_eq!(mat.name, "Default");
        assert_eq!(mat.density, 1.0);
        assert!(mat.description.is_empty());
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Material");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_with_description() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let result = driver.read_from_xml("Aluminum 2.7 Light_Metal");
        assert!(result.is_ok());
        let mat = result.unwrap();
        assert_eq!(mat.name, "Aluminum");
        assert!((mat.density - 2.7).abs() < 1e-10);
        assert_eq!(mat.description, "Light_Metal");
    }

    #[test]
    fn test_read_from_xml_without_description() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let result = driver.read_from_xml("Copper 8.96");
        assert!(result.is_ok());
        let mat = result.unwrap();
        assert_eq!(mat.name, "Copper");
        assert!((mat.density - 8.96).abs() < 1e-10);
        assert!(mat.description.is_empty());
    }

    #[test]
    fn test_read_from_xml_multiword_description() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let result = driver.read_from_xml("Titanium 4.506 High strength alloy");
        assert!(result.is_ok());
        let mat = result.unwrap();
        assert_eq!(mat.name, "Titanium");
        assert_eq!(mat.description, "High strength alloy");
    }

    #[test]
    fn test_read_from_xml_invalid_density() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let result = driver.read_from_xml("Steel not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_missing_density() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let result = driver.read_from_xml("OnlyName");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml_with_description() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let mat = MaterialData::new("Steel", 7.85, "Carbon Steel");
        let xml = driver.write_to_xml(&mat);
        assert_eq!(xml, "Steel 7.85 Carbon Steel");
    }

    #[test]
    fn test_write_to_xml_without_description() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let mut mat = MaterialData::new("Copper", 8.96, "");
        mat.description.clear();
        let xml = driver.write_to_xml(&mat);
        assert_eq!(xml, "Copper 8.96");
    }

    #[test]
    fn test_roundtrip_with_description() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let original = MaterialData::new("Gold", 19.3, "Noble Metal");
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original.name, restored.name);
        assert!((original.density - restored.density).abs() < 1e-10);
        assert_eq!(original.description, restored.description);
    }

    #[test]
    fn test_roundtrip_without_description() {
        let driver = XmlMXCAFDocMaterialDriver::new();
        let mut original = MaterialData::new("Silver", 10.49, "");
        original.description.clear();
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original.name, restored.name);
        assert_eq!(original.description, restored.description);
    }
}
