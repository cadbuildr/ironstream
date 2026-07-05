// FILE: de_shape_fix_configuration_node.rs
// occt: DE_ShapeFixConfigurationNode

/// Base configuration node for shape healing/fixing parameters.
/// Stores settings for shape healing operations used by derived classes.
#[derive(Clone)]
pub struct DeShapeFixConfigurationNode {
    /// Shape healing parameters
    pub shape_fix_parameters: ShapeFixParameters,
}

/// Fix mode enumeration for shape healing procedures
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixMode {
    /// Procedure will be executed or not (depending on the situation)
    FixOrNot = -1,
    /// Procedure will not be executed
    NotFix = 0,
    /// Procedure will be executed anyway
    Fix = 1,
}

impl FixMode {
    /// Convert from integer representation
    pub fn from_int(val: i32) -> Self {
        match val {
            -1 => FixMode::FixOrNot,
            0 => FixMode::NotFix,
            1 => FixMode::Fix,
            _ => FixMode::FixOrNot,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

/// Shape enumeration for detalization level
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeEnum {
    /// Vertex level
    Vertex = 0,
    /// Edge level
    Edge = 1,
    /// Wire level
    Wire = 2,
    /// Face level
    Face = 3,
    /// Shell level
    Shell = 4,
    /// Solid level
    Solid = 5,
    /// Compound level
    Compound = 6,
}

impl ShapeEnum {
    /// Convert from integer representation
    pub fn from_int(val: i32) -> Self {
        match val {
            0 => ShapeEnum::Vertex,
            1 => ShapeEnum::Edge,
            2 => ShapeEnum::Wire,
            3 => ShapeEnum::Face,
            4 => ShapeEnum::Shell,
            5 => ShapeEnum::Solid,
            6 => ShapeEnum::Compound,
            _ => ShapeEnum::Vertex,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

/// Parameters for shape healing and fixing
#[derive(Clone)]
pub struct ShapeFixParameters {
    // Tolerance settings
    pub tolerance_3d: f64,
    pub max_tolerance_3d: f64,
    pub min_tolerance_3d: f64,

    // Geometry/topology settings
    pub detalization_level: ShapeEnum,
    pub non_manifold: bool,

    // Fix mode flags for various operations
    pub fix_free_shell_mode: FixMode,
    pub fix_free_face_mode: FixMode,
    pub fix_free_wire_mode: FixMode,
    pub fix_same_parameter_mode: FixMode,
    pub fix_solid_mode: FixMode,
    pub fix_shell_orientation_mode: FixMode,
    pub create_open_solid_mode: FixMode,
    pub fix_shell_mode: FixMode,
    pub fix_face_orientation_mode: FixMode,
    pub fix_face_mode: FixMode,
    pub fix_wire_mode: FixMode,
    pub fix_orientation_mode: FixMode,
    pub fix_add_natural_bound_mode: FixMode,
    pub fix_missing_seam_mode: FixMode,
    pub fix_small_area_wire_mode: FixMode,
    pub remove_small_area_face_mode: FixMode,
    pub fix_intersecting_wires_mode: FixMode,
    pub fix_loop_wires_mode: FixMode,
    pub fix_split_face_mode: FixMode,
    pub auto_correct_precision_mode: FixMode,
    pub modify_topology_mode: FixMode,
    pub modify_geometry_mode: FixMode,
    pub closed_wire_mode: FixMode,
    pub preference_p_curve_mode: FixMode,
    pub fix_reorder_mode: FixMode,
    pub fix_small_mode: FixMode,
    pub fix_connected_mode: FixMode,
    pub fix_edge_curves_mode: FixMode,
    pub fix_degenerated_mode: FixMode,
    pub fix_lacking_mode: FixMode,
    pub fix_self_intersection_mode: FixMode,
    pub remove_loop_mode: FixMode,
    pub fix_reversed_2d_mode: FixMode,
    pub fix_remove_p_curve_mode: FixMode,
    pub fix_remove_curve_3d_mode: FixMode,
    pub fix_add_p_curve_mode: FixMode,
    pub fix_add_curve_3d_mode: FixMode,
    pub fix_seam_mode: FixMode,
    pub fix_shifted_mode: FixMode,
    pub fix_edge_same_parameter_mode: FixMode,
    pub fix_notched_edges_mode: FixMode,
    pub fix_tail_mode: FixMode,
    pub max_tail_angle_mode: FixMode,
    pub max_tail_width_mode: FixMode,
    pub fix_self_intersecting_edge_mode: FixMode,
    pub fix_intersecting_edges_mode: FixMode,
    pub fix_non_adjacent_intersecting_edges_mode: FixMode,
    pub fix_vertex_position_mode: FixMode,
    pub fix_vertex_tolerance_mode: FixMode,
}

impl Default for ShapeFixParameters {
    fn default() -> Self {
        ShapeFixParameters {
            tolerance_3d: 1.0e-6,
            max_tolerance_3d: 1.0,
            min_tolerance_3d: 1.0e-7,
            detalization_level: ShapeEnum::Vertex,
            non_manifold: false,
            fix_free_shell_mode: FixMode::FixOrNot,
            fix_free_face_mode: FixMode::FixOrNot,
            fix_free_wire_mode: FixMode::FixOrNot,
            fix_same_parameter_mode: FixMode::FixOrNot,
            fix_solid_mode: FixMode::FixOrNot,
            fix_shell_orientation_mode: FixMode::FixOrNot,
            create_open_solid_mode: FixMode::NotFix,
            fix_shell_mode: FixMode::FixOrNot,
            fix_face_orientation_mode: FixMode::FixOrNot,
            fix_face_mode: FixMode::FixOrNot,
            fix_wire_mode: FixMode::FixOrNot,
            fix_orientation_mode: FixMode::FixOrNot,
            fix_add_natural_bound_mode: FixMode::FixOrNot,
            fix_missing_seam_mode: FixMode::FixOrNot,
            fix_small_area_wire_mode: FixMode::FixOrNot,
            remove_small_area_face_mode: FixMode::FixOrNot,
            fix_intersecting_wires_mode: FixMode::FixOrNot,
            fix_loop_wires_mode: FixMode::FixOrNot,
            fix_split_face_mode: FixMode::FixOrNot,
            auto_correct_precision_mode: FixMode::Fix,
            modify_topology_mode: FixMode::NotFix,
            modify_geometry_mode: FixMode::Fix,
            closed_wire_mode: FixMode::Fix,
            preference_p_curve_mode: FixMode::Fix,
            fix_reorder_mode: FixMode::FixOrNot,
            fix_small_mode: FixMode::FixOrNot,
            fix_connected_mode: FixMode::FixOrNot,
            fix_edge_curves_mode: FixMode::FixOrNot,
            fix_degenerated_mode: FixMode::FixOrNot,
            fix_lacking_mode: FixMode::FixOrNot,
            fix_self_intersection_mode: FixMode::FixOrNot,
            remove_loop_mode: FixMode::FixOrNot,
            fix_reversed_2d_mode: FixMode::FixOrNot,
            fix_remove_p_curve_mode: FixMode::FixOrNot,
            fix_remove_curve_3d_mode: FixMode::FixOrNot,
            fix_add_p_curve_mode: FixMode::FixOrNot,
            fix_add_curve_3d_mode: FixMode::FixOrNot,
            fix_seam_mode: FixMode::FixOrNot,
            fix_shifted_mode: FixMode::FixOrNot,
            fix_edge_same_parameter_mode: FixMode::NotFix,
            fix_notched_edges_mode: FixMode::FixOrNot,
            fix_tail_mode: FixMode::NotFix,
            max_tail_angle_mode: FixMode::NotFix,
            max_tail_width_mode: FixMode::FixOrNot,
            fix_self_intersecting_edge_mode: FixMode::FixOrNot,
            fix_intersecting_edges_mode: FixMode::FixOrNot,
            fix_non_adjacent_intersecting_edges_mode: FixMode::FixOrNot,
            fix_vertex_position_mode: FixMode::NotFix,
            fix_vertex_tolerance_mode: FixMode::FixOrNot,
        }
    }
}

impl DeShapeFixConfigurationNode {
    /// Initializes all fields with default values
    pub fn new() -> Self {
        DeShapeFixConfigurationNode {
            shape_fix_parameters: ShapeFixParameters::default(),
        }
    }

    /// Copies values from another configuration node
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Generates configuration string for saving
    pub fn save(&self) -> String {
        let mut result = String::new();
        result.push_str("!*****************************************************************************\n");
        result.push_str("!Shape Fix Configuration Parameters\n");
        result.push_str("!*****************************************************************************\n");

        result.push_str(&format!("tolerance.3d : {}\n", self.shape_fix_parameters.tolerance_3d));
        result.push_str(&format!("max.tolerance.3d : {}\n", self.shape_fix_parameters.max_tolerance_3d));
        result.push_str(&format!("min.tolerance.3d : {}\n", self.shape_fix_parameters.min_tolerance_3d));
        result.push_str(&format!("detalization.level : {}\n", self.shape_fix_parameters.detalization_level.to_int()));
        result.push_str(&format!("non.manifold : {}\n", self.shape_fix_parameters.non_manifold as u8));

        result
    }
}

impl Default for DeShapeFixConfigurationNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_initialization() {
        let params = ShapeFixParameters::default();
        assert_eq!(params.tolerance_3d, 1.0e-6);
        assert_eq!(params.max_tolerance_3d, 1.0);
        assert_eq!(params.min_tolerance_3d, 1.0e-7);
        assert_eq!(params.detalization_level, ShapeEnum::Vertex);
        assert!(!params.non_manifold);
    }

    #[test]
    fn test_fix_mode_conversion() {
        assert_eq!(FixMode::from_int(-1), FixMode::FixOrNot);
        assert_eq!(FixMode::from_int(0), FixMode::NotFix);
        assert_eq!(FixMode::from_int(1), FixMode::Fix);
        assert_eq!(FixMode::FixOrNot.to_int(), -1);
        assert_eq!(FixMode::NotFix.to_int(), 0);
        assert_eq!(FixMode::Fix.to_int(), 1);
    }

    #[test]
    fn test_shape_enum_conversion() {
        assert_eq!(ShapeEnum::from_int(0), ShapeEnum::Vertex);
        assert_eq!(ShapeEnum::from_int(1), ShapeEnum::Edge);
        assert_eq!(ShapeEnum::from_int(5), ShapeEnum::Solid);
        assert_eq!(ShapeEnum::Vertex.to_int(), 0);
        assert_eq!(ShapeEnum::Face.to_int(), 3);
    }

    #[test]
    fn test_node_creation() {
        let node = DeShapeFixConfigurationNode::new();
        assert_eq!(node.shape_fix_parameters.tolerance_3d, 1.0e-6);
    }

    #[test]
    fn test_node_copy() {
        let node = DeShapeFixConfigurationNode::new();
        let copied = node.copy();
        assert_eq!(copied.shape_fix_parameters.tolerance_3d, node.shape_fix_parameters.tolerance_3d);
    }

    #[test]
    fn test_fix_mode_defaults() {
        let params = ShapeFixParameters::default();
        assert_eq!(params.fix_free_shell_mode, FixMode::FixOrNot);
        assert_eq!(params.create_open_solid_mode, FixMode::NotFix);
        assert_eq!(params.auto_correct_precision_mode, FixMode::Fix);
    }

    #[test]
    fn test_save_configuration() {
        let node = DeShapeFixConfigurationNode::new();
        let config = node.save();
        assert!(config.contains("tolerance.3d"));
        assert!(config.contains("max.tolerance.3d"));
        assert!(config.contains("detalization.level"));
    }
}
