// FILE: int_tools_topol_tool.rs
// occt: IntTools_TopolTool

/// Topological tool for surface sampling.
#[derive(Clone, Debug)]
pub struct IntToolsTopolTool {
    nb_smpl_u: i32,
    nb_smpl_v: i32,
    u0: f64,
    v0: f64,
    du: f64,
    dv: f64,
}

impl IntToolsTopolTool {
    /// Create an empty topol tool.
    pub fn new() -> Self {
        IntToolsTopolTool {
            nb_smpl_u: 0,
            nb_smpl_v: 0,
            u0: 0.0,
            v0: 0.0,
            du: 0.0,
            dv: 0.0,
        }
    }

    /// Get U samples.
    pub fn nb_samples_u(&self) -> i32 {
        self.nb_smpl_u
    }

    /// Get V samples.
    pub fn nb_samples_v(&self) -> i32 {
        self.nb_smpl_v
    }

    /// Get total samples.
    pub fn nb_samples(&self) -> i32 {
        self.nb_smpl_u * self.nb_smpl_v
    }

    /// Compute sample points (placeholder).
    pub fn compute_sample_points(&mut self) {
        self.nb_smpl_u = 10;
        self.nb_smpl_v = 10;
    }
}

impl Default for IntToolsTopolTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = IntToolsTopolTool::new();
        assert_eq!(tool.nb_samples_u(), 0);
        assert_eq!(tool.nb_samples_v(), 0);
    }

    #[test]
    fn test_compute_sample_points() {
        let mut tool = IntToolsTopolTool::new();
        tool.compute_sample_points();
        assert_eq!(tool.nb_samples_u(), 10);
        assert_eq!(tool.nb_samples_v(), 10);
    }

    #[test]
    fn test_nb_samples() {
        let mut tool = IntToolsTopolTool::new();
        tool.compute_sample_points();
        assert_eq!(tool.nb_samples(), 100);
    }
}
