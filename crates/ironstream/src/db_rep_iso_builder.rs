// FILE: db_rep_iso_builder.rs
// occt: DBRep_IsoBuilder

/// Represents a 2D parametric curve
#[derive(Debug, Clone)]
pub struct Curve2D {
    pub id: u64,
    pub control_points: Vec<[f64; 2]>,
}

impl Curve2D {
    pub fn new(id: u64) -> Self {
        Curve2D {
            id,
            control_points: Vec::new(),
        }
    }
}

/// Represents a 2D face/shape stub
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape2D {
    pub id: u64,
}

impl Default for Shape2D {
    fn default() -> Self {
        Shape2D { id: 0 }
    }
}

/// Parameter range
#[derive(Debug, Clone, Copy)]
pub struct ParameterRange {
    pub min: f64,
    pub max: f64,
}

impl ParameterRange {
    pub fn new(min: f64, max: f64) -> Self {
        ParameterRange { min, max }
    }
}

/// Isoparametric curve builder for creating ISO parametric lines on faces.
pub struct DBRepIsoBuilder {
    u_range: ParameterRange,
    v_range: ParameterRange,
    infinite: f64,
    nb_isos: usize,
    u_params: Vec<f64>,
    v_params: Vec<f64>,
    u_indices: Vec<usize>,
    v_indices: Vec<usize>,
    nb_domains: usize,
}

impl DBRepIsoBuilder {
    /// Create a new ISO builder with the given face bounds and number of ISOs.
    pub fn new(
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        infinite: f64,
        nb_isos: usize,
    ) -> Self {
        let u_range = ParameterRange::new(u_min, u_max);
        let v_range = ParameterRange::new(v_min, v_max);

        let mut builder = DBRepIsoBuilder {
            u_range,
            v_range,
            infinite,
            nb_isos,
            u_params: Vec::new(),
            v_params: Vec::new(),
            u_indices: Vec::new(),
            v_indices: Vec::new(),
            nb_domains: 0,
        };

        builder.initialize_parameters();
        builder
    }

    /// Initialize parameter arrays based on the ranges and number of ISOs.
    fn initialize_parameters(&mut self) {
        if self.nb_isos == 0 {
            self.nb_domains = 0;
            return;
        }

        let u_step = (self.u_range.max - self.u_range.min) / (self.nb_isos + 1) as f64;
        let v_step = (self.v_range.max - self.v_range.min) / (self.nb_isos + 1) as f64;

        self.u_params.clear();
        self.v_params.clear();
        self.u_indices.clear();
        self.v_indices.clear();

        for i in 1..=self.nb_isos {
            let u = self.u_range.min + u_step * i as f64;
            let v = self.v_range.min + v_step * i as f64;
            self.u_params.push(u);
            self.v_params.push(v);
            self.u_indices.push(i);
            self.v_indices.push(i);
        }

        self.nb_domains = (self.nb_isos + 1) * (self.nb_isos + 1);
    }

    /// Get the total number of domains (regions between ISO lines).
    pub fn nb_domains(&self) -> usize {
        self.nb_domains
    }

    /// Get the U parameter range.
    pub fn u_range(&self) -> ParameterRange {
        self.u_range
    }

    /// Get the V parameter range.
    pub fn v_range(&self) -> ParameterRange {
        self.v_range
    }

    /// Get the number of ISOs in each direction.
    pub fn nb_isos(&self) -> usize {
        self.nb_isos
    }

    /// Get the infinite distance.
    pub fn infinite(&self) -> f64 {
        self.infinite
    }

    /// Get U parameters.
    pub fn u_params(&self) -> &[f64] {
        &self.u_params
    }

    /// Get V parameters.
    pub fn v_params(&self) -> &[f64] {
        &self.v_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = DBRepIsoBuilder::new(0.0, 10.0, 0.0, 5.0, 1e15, 3);
        assert_eq!(builder.nb_isos(), 3);
        assert_eq!(builder.nb_domains(), 16); // (3+1) * (3+1)
        assert_eq!(builder.infinite(), 1e15);
    }

    #[test]
    fn test_parameter_ranges() {
        let builder = DBRepIsoBuilder::new(0.0, 10.0, 1.0, 6.0, 1e15, 3);
        let u_range = builder.u_range();
        let v_range = builder.v_range();

        assert_eq!(u_range.min, 0.0);
        assert_eq!(u_range.max, 10.0);
        assert_eq!(v_range.min, 1.0);
        assert_eq!(v_range.max, 6.0);
    }

    #[test]
    fn test_parameters_distribution() {
        let builder = DBRepIsoBuilder::new(0.0, 10.0, 0.0, 5.0, 1e15, 3);
        let u_params = builder.u_params();
        let v_params = builder.v_params();

        assert_eq!(u_params.len(), 3);
        assert_eq!(v_params.len(), 3);

        // Check that parameters are evenly distributed
        let u_expected = vec![2.5, 5.0, 7.5];
        let v_expected = vec![1.25, 2.5, 3.75];

        for (i, &p) in u_params.iter().enumerate() {
            assert!((p - u_expected[i]).abs() < 1e-10);
        }

        for (i, &p) in v_params.iter().enumerate() {
            assert!((p - v_expected[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_zero_isos() {
        let builder = DBRepIsoBuilder::new(0.0, 10.0, 0.0, 5.0, 1e15, 0);
        assert_eq!(builder.nb_isos(), 0);
        assert_eq!(builder.nb_domains(), 0);
        assert_eq!(builder.u_params().len(), 0);
    }

    #[test]
    fn test_single_iso() {
        let builder = DBRepIsoBuilder::new(0.0, 10.0, 0.0, 5.0, 1e15, 1);
        assert_eq!(builder.nb_isos(), 1);
        assert_eq!(builder.nb_domains(), 4); // (1+1) * (1+1)
        assert_eq!(builder.u_params().len(), 1);
        assert_eq!(builder.v_params().len(), 1);
    }
}
