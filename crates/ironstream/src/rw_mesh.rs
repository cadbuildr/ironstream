// FILE: rw_mesh.rs
// occt: RWMesh

use crate::rw_mesh_name_format::RWMeshNameFormat;

/// Auxiliary tools for RWMesh package.
pub struct RWMesh;

impl RWMesh {
    /// Read name attribute from a label.
    /// In the actual implementation, this would read from a TDF_Label structure.
    pub fn read_name_attribute(label_id: usize) -> String {
        // Placeholder: actual implementation would access TDF_Label
        format!("Label_{}", label_id)
    }

    /// Generate name for specified labels.
    ///
    /// # Parameters
    /// - `format`: name format to apply
    /// - `label_id`: instance label identifier
    /// - `ref_label_id`: product label identifier
    pub fn format_name(format: RWMeshNameFormat, label_id: usize, ref_label_id: usize) -> String {
        match format {
            RWMeshNameFormat::Empty => String::new(),
            RWMeshNameFormat::Product => format!("Product_{}", ref_label_id),
            RWMeshNameFormat::Instance => format!("Instance_{}", label_id),
            RWMeshNameFormat::InstanceOrProduct => {
                if label_id > 0 {
                    format!("Instance_{}", label_id)
                } else {
                    format!("Product_{}", ref_label_id)
                }
            }
            RWMeshNameFormat::ProductOrInstance => {
                if ref_label_id > 0 {
                    format!("Product_{}", ref_label_id)
                } else {
                    format!("Instance_{}", label_id)
                }
            }
            RWMeshNameFormat::ProductAndInstance => {
                format!("Product_{} [Instance_{}]", ref_label_id, label_id)
            }
            RWMeshNameFormat::ProductAndInstanceAndOcaf => {
                format!("Product_{}_Instance_{}_OCAF", ref_label_id, label_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_name_attribute() {
        let name = RWMesh::read_name_attribute(42);
        assert_eq!(name, "Label_42");
    }

    #[test]
    fn test_format_name_empty() {
        let name = RWMesh::format_name(RWMeshNameFormat::Empty, 1, 2);
        assert_eq!(name, "");
    }

    #[test]
    fn test_format_name_product() {
        let name = RWMesh::format_name(RWMeshNameFormat::Product, 1, 2);
        assert_eq!(name, "Product_2");
    }

    #[test]
    fn test_format_name_instance() {
        let name = RWMesh::format_name(RWMeshNameFormat::Instance, 5, 10);
        assert_eq!(name, "Instance_5");
    }

    #[test]
    fn test_format_name_instance_or_product() {
        let name1 = RWMesh::format_name(RWMeshNameFormat::InstanceOrProduct, 5, 10);
        assert_eq!(name1, "Instance_5");

        let name2 = RWMesh::format_name(RWMeshNameFormat::InstanceOrProduct, 0, 10);
        assert_eq!(name2, "Product_10");
    }

    #[test]
    fn test_format_name_product_or_instance() {
        let name1 = RWMesh::format_name(RWMeshNameFormat::ProductOrInstance, 5, 10);
        assert_eq!(name1, "Product_10");

        let name2 = RWMesh::format_name(RWMeshNameFormat::ProductOrInstance, 5, 0);
        assert_eq!(name2, "Instance_5");
    }

    #[test]
    fn test_format_name_product_and_instance() {
        let name = RWMesh::format_name(RWMeshNameFormat::ProductAndInstance, 5, 10);
        assert_eq!(name, "Product_10 [Instance_5]");
    }

    #[test]
    fn test_format_name_product_and_instance_and_ocaf() {
        let name = RWMesh::format_name(RWMeshNameFormat::ProductAndInstanceAndOcaf, 5, 10);
        assert_eq!(name, "Product_10_Instance_5_OCAF");
    }
}
