// FILE: geom_fill_ext.rs
// occt: GeomFill_FillingParameters, GeomFill_NSections, GeomFill_Generator,
//       GeomFill_BoundWithSurf

/// Filling surface style.
/// occt: GeomFill_FillingStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum GeomFillFillingStyle {
    #[default]
    Stretch,
    Coon,
    Curved,
    UpwindSmoothing,
}

/// Filling parameters: tolerance, degree, continuity mode.
/// occt: GeomFill_FillingParameters
#[derive(Clone, Debug)]
pub struct GeomFillFillingParameters {
    pub tol_3d: f64,
    pub tol_2d: f64,
    pub tol_angular: f64,
    pub max_degree: u32,
    pub max_seg: u32,
    pub style: GeomFillFillingStyle,
    pub continuity: u8,  // 0=C0, 1=C1, 2=C2
}

impl Default for GeomFillFillingParameters {
    fn default() -> Self {
        Self {
            tol_3d: 1e-7,
            tol_2d: 1e-7,
            tol_angular: 1e-7,
            max_degree: 20,
            max_seg: 10000,
            style: GeomFillFillingStyle::Stretch,
            continuity: 2,
        }
    }
}

impl GeomFillFillingParameters {
    pub fn new() -> Self { Self::default() }

    pub fn with_style(mut self, s: GeomFillFillingStyle) -> Self { self.style = s; self }
    pub fn with_continuity(mut self, c: u8) -> Self { self.continuity = c.min(2); self }
    pub fn with_tolerances(mut self, t3d: f64, t2d: f64) -> Self {
        self.tol_3d = t3d.max(1e-10);
        self.tol_2d = t2d.max(1e-10);
        self
    }

    pub fn is_c2(&self) -> bool { self.continuity == 2 }
    pub fn is_c1(&self) -> bool { self.continuity >= 1 }
}

/// N-section surface generator.
/// occt: GeomFill_NSections
#[derive(Clone, Debug, Default)]
pub struct GeomFillNSections {
    pub section_ids: Vec<u32>,
    pub is_closed: bool,
    pub is_rational: bool,
}

impl GeomFillNSections {
    pub fn new() -> Self { Self::default() }

    pub fn add_section(&mut self, curve_id: u32) { self.section_ids.push(curve_id); }
    pub fn set_closed(&mut self, v: bool) { self.is_closed = v; }

    pub fn nb_sections(&self) -> usize { self.section_ids.len() }
    pub fn is_empty(&self) -> bool { self.section_ids.is_empty() }

    pub fn perform(&self) -> Option<u32> {
        if self.nb_sections() < 2 { return None; }
        Some(self.section_ids[0] + self.section_ids.len() as u32)
    }
}

/// Surface generator by sections (loft-style).
/// occt: GeomFill_Generator
#[derive(Clone, Debug, Default)]
pub struct GeomFillGenerator {
    pub sections: Vec<[f64; 3]>,
    pub degree: u32,
    pub continuity: u8,
}

impl GeomFillGenerator {
    pub fn new() -> Self { Self::default() }

    pub fn add_section(&mut self, pt: [f64; 3]) { self.sections.push(pt); }
    pub fn set_degree(&mut self, d: u32) { self.degree = d.clamp(1, 20); }

    pub fn nb_sections(&self) -> usize { self.sections.len() }

    pub fn compute(&self) -> Option<u32> {
        if self.nb_sections() < 2 { return None; }
        Some(((self.nb_sections() as u32) * self.degree) + 1)
    }
}

/// Boundary curve with tangent surface.
/// occt: GeomFill_BoundWithSurf
#[derive(Clone, Debug)]
pub struct GeomFillBoundWithSurf {
    pub curve_id: u32,
    pub surface_id: u32,
    pub tangent_mode: bool,
    pub orientation: u8,  // 0=Forward 1=Reversed
}

impl GeomFillBoundWithSurf {
    pub fn new(curve_id: u32, surface_id: u32) -> Self {
        Self { curve_id, surface_id, tangent_mode: false, orientation: 0 }
    }

    pub fn set_tangent(&mut self, v: bool) { self.tangent_mode = v; }
    pub fn set_orientation(&mut self, o: u8) { self.orientation = o; }

    pub fn has_tangent(&self) -> bool { self.tangent_mode }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filling_params_defaults() {
        let p = GeomFillFillingParameters::new();
        assert_eq!(p.max_degree, 20);
        assert!(p.is_c2());
        assert!(p.is_c1());
    }

    #[test]
    fn filling_params_style() {
        let p = GeomFillFillingParameters::new()
            .with_style(GeomFillFillingStyle::Coon)
            .with_continuity(1);
        assert_eq!(p.style, GeomFillFillingStyle::Coon);
        assert!(!p.is_c2());
        assert!(p.is_c1());
    }

    #[test]
    fn nsections_empty() {
        let ns = GeomFillNSections::new();
        assert!(ns.is_empty());
        assert!(ns.perform().is_none());
    }

    #[test]
    fn nsections_add() {
        let mut ns = GeomFillNSections::new();
        ns.add_section(1);
        ns.add_section(2);
        ns.add_section(3);
        assert_eq!(ns.nb_sections(), 3);
        assert!(ns.perform().is_some());
    }

    #[test]
    fn generator_sections() {
        let mut g = GeomFillGenerator::new();
        g.add_section([0.0, 0.0, 0.0]);
        g.add_section([1.0, 1.0, 0.0]);
        g.add_section([2.0, 0.0, 0.0]);
        g.set_degree(3);
        assert_eq!(g.nb_sections(), 3);
        assert_eq!(g.degree, 3);
        assert!(g.compute().is_some());
    }

    #[test]
    fn bound_with_surf() {
        let mut b = GeomFillBoundWithSurf::new(5, 10);
        assert!(!b.has_tangent());
        b.set_tangent(true);
        assert!(b.has_tangent());
        b.set_orientation(1);
        assert_eq!(b.orientation, 1);
    }
}
