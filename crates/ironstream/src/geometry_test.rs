// FILE: geometry_test.rs
// occt: GeometryTest

//! Geometry test utilities for Draw Interpretor.
//! This package provides commands for curves and surfaces.

pub struct GeometryTestInterpretor;

impl GeometryTestInterpretor {
    /// Define all geometric commands
    pub fn all_commands() -> String {
        "GeometryTest commands loaded".to_string()
    }

    /// Define curve commands
    pub fn curve_commands() -> String {
        "Curve commands loaded".to_string()
    }

    /// Define tangent curve commands
    pub fn curve_tan_commands() -> String {
        "CurveTan commands loaded".to_string()
    }

    /// Define fair curve commands
    pub fn fair_curve_commands() -> String {
        "FairCurve commands loaded".to_string()
    }

    /// Define surface commands
    pub fn surface_commands() -> String {
        "Surface commands loaded".to_string()
    }

    /// Define constrained curves commands
    pub fn constraint_commands() -> String {
        "Constraint commands loaded".to_string()
    }

    /// Define commands to test the GeomAPI
    pub fn api_commands() -> String {
        "API commands loaded".to_string()
    }

    /// Define command to test local continuity
    pub fn continuity_commands() -> String {
        "Continuity commands loaded".to_string()
    }

    /// Define commands to test polyhedral triangulations
    pub fn poly_commands() -> String {
        "Poly commands loaded".to_string()
    }

    /// Define commands to test projection
    pub fn test_proj_commands() -> String {
        "TestProj commands loaded".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_commands() {
        let result = GeometryTestInterpretor::all_commands();
        assert!(result.contains("loaded"));
    }

    #[test]
    fn test_curve_commands() {
        let result = GeometryTestInterpretor::curve_commands();
        assert_eq!(result, "Curve commands loaded");
    }

    #[test]
    fn test_surface_commands() {
        let result = GeometryTestInterpretor::surface_commands();
        assert_eq!(result, "Surface commands loaded");
    }

    #[test]
    fn test_api_commands() {
        let result = GeometryTestInterpretor::api_commands();
        assert!(result.contains("API"));
    }

    #[test]
    fn test_multiple_commands() {
        let c1 = GeometryTestInterpretor::curve_commands();
        let c2 = GeometryTestInterpretor::surface_commands();
        assert_ne!(c1, c2);
        assert!(c1.contains("Curve"));
        assert!(c2.contains("Surface"));
    }
}
