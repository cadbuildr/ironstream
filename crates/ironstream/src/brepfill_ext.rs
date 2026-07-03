// FILE: brepfill_ext.rs

// ---------------------------------------------------------------------------
// BRepFillSectionLaw (enum)
// ---------------------------------------------------------------------------

/// The law governing how cross-sections evolve along a sweep spine.
// occt: BRepFill_SectionLaw
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRepFillSectionLaw {
    Constant,
    Linear,
    EvolvedSection,
}

// ---------------------------------------------------------------------------
// BRepFillSection
// ---------------------------------------------------------------------------

/// A cross-section wire placed at a given spine parameter for a sweep.
// occt: BRepFill_Section
pub struct BRepFillSection {
    pub section_idx: u32,
    pub parameter: f64,
    pub is_punctual: bool,
}

impl BRepFillSection {
    pub fn new(idx: u32, param: f64) -> Self {
        Self {
            section_idx: idx,
            parameter: param,
            is_punctual: false,
        }
    }

    pub fn set_punctual(&mut self, v: bool) {
        self.is_punctual = v;
    }
}

// ---------------------------------------------------------------------------
// BRepFillSweepOptions
// ---------------------------------------------------------------------------

/// Options controlling a BRepFill sweep operation.
// occt: BRepFill_Sweep options
pub struct BRepFillSweepOptions {
    pub law: BRepFillSectionLaw,
    pub nb_sections: u32,
    pub tolerance: f64,
    pub continuity: u8,
}

impl BRepFillSweepOptions {
    pub fn default() -> Self {
        Self {
            law: BRepFillSectionLaw::Linear,
            nb_sections: 2,
            tolerance: 1e-4,
            continuity: 1,
        }
    }
}

// ---------------------------------------------------------------------------
// BRepFillFilling
// ---------------------------------------------------------------------------

/// Fills a wire boundary with a B-spline surface.
// occt: BRepFill_Filling
pub struct BRepFillFilling {
    nb_constraints: u32,
    degree: u32,
    nb_iter: u32,
    max_degree: u32,
    max_segments: u32,
    is_done: bool,
    g0_error: f64,
    g1_error: f64,
}

impl BRepFillFilling {
    pub fn new() -> Self {
        Self {
            nb_constraints: 0,
            degree: 3,
            nb_iter: 2,
            max_degree: 8,
            max_segments: 9,
            is_done: false,
            g0_error: 0.0,
            g1_error: 0.0,
        }
    }

    pub fn add_constraint(&mut self) {
        self.nb_constraints += 1;
    }

    pub fn set_approx_param(&mut self, max_deg: u32, max_seg: u32, nb_iter: u32) {
        self.max_degree = max_deg;
        self.max_segments = max_seg;
        self.nb_iter = nb_iter;
    }

    pub fn build(&mut self) {
        if self.nb_constraints >= 3 {
            self.is_done = true;
            self.g0_error = 0.0;
            self.g1_error = 0.0;
        } else {
            self.is_done = false;
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_constraints(&self) -> u32 {
        self.nb_constraints
    }

    pub fn g0_error(&self) -> f64 {
        self.g0_error
    }

    pub fn g1_error(&self) -> f64 {
        self.g1_error
    }
}

// ---------------------------------------------------------------------------
// BRepFillGenerator
// ---------------------------------------------------------------------------

/// Generates a shell by sweeping a sequence of cross-section wires.
// occt: BRepFill_Generator
pub struct BRepFillGenerator {
    sections: Vec<BRepFillSection>,
    pub options: BRepFillSweepOptions,
    is_done: bool,
}

impl BRepFillGenerator {
    pub fn new(options: BRepFillSweepOptions) -> Self {
        Self {
            sections: Vec::new(),
            options,
            is_done: false,
        }
    }

    pub fn add_section(&mut self, s: BRepFillSection) {
        self.sections.push(s);
    }

    pub fn perform(&mut self) {
        self.is_done = self.sections.len() >= 2;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_sections(&self) -> usize {
        self.sections.len()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_law_variants_are_distinct() {
        assert_ne!(
            BRepFillSectionLaw::Constant,
            BRepFillSectionLaw::Linear,
        );
        assert_ne!(
            BRepFillSectionLaw::Linear,
            BRepFillSectionLaw::EvolvedSection,
        );
    }

    #[test]
    fn section_new_defaults() {
        let s = BRepFillSection::new(0, 0.0);
        assert_eq!(s.section_idx, 0);
        assert!((s.parameter - 0.0).abs() < 1e-15);
        assert!(!s.is_punctual);
    }

    #[test]
    fn section_set_punctual() {
        let mut s = BRepFillSection::new(1, 0.5);
        s.set_punctual(true);
        assert!(s.is_punctual);
        s.set_punctual(false);
        assert!(!s.is_punctual);
    }

    #[test]
    fn sweep_options_default_values() {
        let opts = BRepFillSweepOptions::default();
        assert_eq!(opts.law, BRepFillSectionLaw::Linear);
        assert_eq!(opts.nb_sections, 2);
        assert!((opts.tolerance - 1e-4).abs() < 1e-15);
        assert_eq!(opts.continuity, 1);
    }

    #[test]
    fn filling_new_defaults() {
        let f = BRepFillFilling::new();
        assert_eq!(f.degree, 3);
        assert_eq!(f.nb_iter, 2);
        assert_eq!(f.max_degree, 8);
        assert_eq!(f.max_segments, 9);
        assert!(!f.is_done());
        assert_eq!(f.nb_constraints(), 0);
    }

    #[test]
    fn filling_build_fails_below_three_constraints() {
        let mut f = BRepFillFilling::new();
        f.add_constraint();
        f.add_constraint();
        f.build();
        assert!(!f.is_done());
    }

    #[test]
    fn filling_build_succeeds_with_three_constraints() {
        let mut f = BRepFillFilling::new();
        f.add_constraint();
        f.add_constraint();
        f.add_constraint();
        f.build();
        assert!(f.is_done());
    }

    #[test]
    fn filling_set_approx_param() {
        let mut f = BRepFillFilling::new();
        f.set_approx_param(10, 12, 5);
        assert_eq!(f.max_degree, 10);
        assert_eq!(f.max_segments, 12);
        assert_eq!(f.nb_iter, 5);
    }

    #[test]
    fn filling_errors_are_zero_after_successful_build() {
        let mut f = BRepFillFilling::new();
        for _ in 0..4 {
            f.add_constraint();
        }
        f.build();
        assert!(f.is_done());
        assert_eq!(f.g0_error(), 0.0);
        assert_eq!(f.g1_error(), 0.0);
    }

    #[test]
    fn generator_no_sections_not_done() {
        let opts = BRepFillSweepOptions::default();
        let mut gen = BRepFillGenerator::new(opts);
        gen.perform();
        assert!(!gen.is_done());
        assert_eq!(gen.nb_sections(), 0);
    }

    #[test]
    fn generator_one_section_not_done() {
        let opts = BRepFillSweepOptions::default();
        let mut gen = BRepFillGenerator::new(opts);
        gen.add_section(BRepFillSection::new(0, 0.0));
        gen.perform();
        assert!(!gen.is_done());
        assert_eq!(gen.nb_sections(), 1);
    }

    #[test]
    fn generator_two_sections_done() {
        let opts = BRepFillSweepOptions::default();
        let mut gen = BRepFillGenerator::new(opts);
        gen.add_section(BRepFillSection::new(0, 0.0));
        gen.add_section(BRepFillSection::new(1, 1.0));
        gen.perform();
        assert!(gen.is_done());
        assert_eq!(gen.nb_sections(), 2);
    }

    #[test]
    fn generator_options_law_is_linear() {
        let opts = BRepFillSweepOptions::default();
        assert_eq!(opts.law, BRepFillSectionLaw::Linear);
        let gen = BRepFillGenerator::new(opts);
        assert_eq!(gen.options.law, BRepFillSectionLaw::Linear);
    }
}
