// FILE: iges_geom_plane.rs
// occt: IGESGeom_Plane

/// Defines IGESPlane, Type <108> Form <-1, 0, 1> in package IGESGeom.
/// A plane entity can represent both unbounded planes and bounded portions of planes.
/// The plane is defined by coefficients A, B, C, D where A*X + B*Y + C*Z = D.
#[derive(Clone, Debug)]
pub struct Plane {
    /// Plane equation coefficient A
    coeff_a: f64,
    /// Plane equation coefficient B
    coeff_b: f64,
    /// Plane equation coefficient C
    coeff_c: f64,
    /// Plane equation coefficient D
    coeff_d: f64,
    /// Optional bounding curve entity ID
    bounding_curve_id: Option<i32>,
    /// Symbol attachment point (x, y, z)
    symbol_attach: [f64; 3],
    /// Size of optional display symbol
    symbol_size: f64,
    /// Form number: 0 (no bound), 1 (external bound), -1 (hole)
    form: i32,
    /// Entity type for IGES (always 108)
    entity_type: i32,
}

impl Plane {
    /// Creates a new Plane entity.
    pub fn new() -> Self {
        Plane {
            coeff_a: 0.0,
            coeff_b: 0.0,
            coeff_c: 1.0,
            coeff_d: 0.0,
            bounding_curve_id: None,
            symbol_attach: [0.0, 0.0, 0.0],
            symbol_size: 0.0,
            form: 0,
            entity_type: 108,
        }
    }

    /// Initializes the Plane with equation coefficients, optional bounding curve, and symbol.
    pub fn init(
        &mut self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        curve: Option<i32>,
        attach: [f64; 3],
        size: f64,
    ) {
        self.coeff_a = a;
        self.coeff_b = b;
        self.coeff_c = c;
        self.coeff_d = d;
        self.bounding_curve_id = curve;
        self.symbol_attach = attach;
        self.symbol_size = size;
        self.form = 0;
    }

    /// Sets the form number (0 = no bound, 1 = external bound, -1 = hole).
    pub fn set_form_number(&mut self, form: i32) {
        if form == 0 || form == 1 || form == -1 || (form >= 10 && form <= 12) {
            self.form = form;
        }
    }

    /// Returns the plane equation coefficients.
    pub fn equation(&self) -> (f64, f64, f64, f64) {
        (self.coeff_a, self.coeff_b, self.coeff_c, self.coeff_d)
    }

    /// Returns the plane equation after transformation.
    pub fn transformed_equation(&self) -> (f64, f64, f64, f64) {
        // TODO: Apply transformation matrix if present
        (self.coeff_a, self.coeff_b, self.coeff_c, self.coeff_d)
    }

    /// Returns true if there exists a bounding curve.
    pub fn has_bounding_curve(&self) -> bool {
        self.bounding_curve_id.is_some()
    }

    /// Returns true if bounding curve exists and bounded portion is negative (hole).
    pub fn has_bounding_curve_hole(&self) -> bool {
        self.bounding_curve_id.is_some() && self.form == -1
    }

    /// Returns the optional bounding curve entity ID.
    pub fn bounding_curve(&self) -> Option<i32> {
        self.bounding_curve_id
    }

    /// Returns true if symbol size is greater than 0.
    pub fn has_symbol_attach(&self) -> bool {
        self.symbol_size > 0.0
    }

    /// Returns the symbol attachment point.
    pub fn symbol_attach(&self) -> [f64; 3] {
        if self.has_symbol_attach() {
            self.symbol_attach
        } else {
            [0.0, 0.0, 0.0]
        }
    }

    /// Returns the symbol attachment point after transformation.
    pub fn transformed_symbol_attach(&self) -> [f64; 3] {
        // TODO: Apply transformation matrix if present
        self.symbol_attach()
    }

    /// Returns the symbol size.
    pub fn symbol_size(&self) -> f64 {
        self.symbol_size
    }

    /// Returns the form number.
    pub fn form_number(&self) -> i32 {
        self.form
    }

    /// Returns the entity type number (always 108).
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_plane() {
        let plane = Plane::new();
        let (a, b, c, d) = plane.equation();
        assert_eq!(a, 0.0);
        assert_eq!(b, 0.0);
        assert_eq!(c, 1.0);
        assert_eq!(d, 0.0);
        assert!(!plane.has_bounding_curve());
        assert!(!plane.has_symbol_attach());
    }

    #[test]
    fn test_init_plane() {
        let mut plane = Plane::new();
        plane.init(1.0, 0.0, 0.0, 5.0, None, [0.0, 0.0, 0.0], 0.0);

        let (a, b, c, d) = plane.equation();
        assert_eq!(a, 1.0);
        assert_eq!(b, 0.0);
        assert_eq!(c, 0.0);
        assert_eq!(d, 5.0);
    }

    #[test]
    fn test_plane_with_bounding_curve() {
        let mut plane = Plane::new();
        plane.init(0.0, 0.0, 1.0, 0.0, Some(1), [1.0, 2.0, 0.0], 0.5);
        plane.set_form_number(1);

        assert!(plane.has_bounding_curve());
        assert_eq!(plane.bounding_curve(), Some(1));
        assert_eq!(plane.form_number(), 1);
    }

    #[test]
    fn test_plane_with_hole() {
        let mut plane = Plane::new();
        plane.init(0.0, 0.0, 1.0, 0.0, Some(2), [0.0, 0.0, 0.0], 0.0);
        plane.set_form_number(-1);

        assert!(plane.has_bounding_curve());
        assert!(plane.has_bounding_curve_hole());
        assert_eq!(plane.form_number(), -1);
    }

    #[test]
    fn test_symbol_attachment() {
        let mut plane = Plane::new();
        plane.init(0.0, 0.0, 1.0, 0.0, None, [3.0, 4.0, 5.0], 2.0);

        assert!(plane.has_symbol_attach());
        assert_eq!(plane.symbol_size(), 2.0);
        assert_eq!(plane.symbol_attach(), [3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_form_number_validation() {
        let mut plane = Plane::new();

        // Valid forms
        plane.set_form_number(0);
        assert_eq!(plane.form_number(), 0);

        plane.set_form_number(1);
        assert_eq!(plane.form_number(), 1);

        plane.set_form_number(-1);
        assert_eq!(plane.form_number(), -1);

        plane.set_form_number(11);
        assert_eq!(plane.form_number(), 11);

        // Invalid form should not change
        plane.set_form_number(5);
        assert_eq!(plane.form_number(), 11);
    }
}
