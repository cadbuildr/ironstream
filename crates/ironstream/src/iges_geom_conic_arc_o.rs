// FILE: iges_geom_conic_arc_o.rs
// occt: IGESGeom_ConicArc

pub struct IgesGeomConicArc {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    z_t: f64,
    start: (f64, f64),
    end: (f64, f64),
}

impl IgesGeomConicArc {
    pub fn new() -> Self {
        IgesGeomConicArc {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 0.0,
            z_t: 0.0,
            start: (0.0, 0.0),
            end: (0.0, 0.0),
        }
    }

    pub fn init(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, z_t: f64, start: (f64, f64), end: (f64, f64)) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
        self.e = e;
        self.f = f;
        self.z_t = z_t;
        self.start = start;
        self.end = end;
    }

    pub fn own_correct(&mut self) -> bool {
        let form = self.computed_form_number();
        // Form number is embedded in the entity; default implementation
        form > 0
    }

    pub fn computed_form_number(&self) -> i32 {
        // Form 1 = Ellipse (B^2 - 4AC < 0, AC > 0)
        // Form 2 = Hyperbola (B^2 - 4AC > 0)
        // Form 3 = Parabola (B^2 - 4AC = 0)
        let discriminant = self.b * self.b - 4.0 * self.a * self.c;
        if (discriminant).abs() < 1e-10 {
            3 // Parabola
        } else if discriminant < 0.0 && self.a * self.c > 0.0 {
            1 // Ellipse
        } else if discriminant > 0.0 {
            2 // Hyperbola
        } else {
            0 // Unknown
        }
    }

    pub fn equation(&self) -> (f64, f64, f64, f64, f64, f64) {
        (self.a, self.b, self.c, self.d, self.e, self.f)
    }

    pub fn z_plane(&self) -> f64 {
        self.z_t
    }

    pub fn start_point(&self) -> (f64, f64) {
        self.start
    }

    pub fn end_point(&self) -> (f64, f64) {
        self.end
    }

    pub fn is_from_ellipse(&self) -> bool {
        self.computed_form_number() == 1
    }

    pub fn is_from_parabola(&self) -> bool {
        self.computed_form_number() == 3
    }

    pub fn is_from_hyperbola(&self) -> bool {
        self.computed_form_number() == 2
    }

    pub fn is_closed(&self) -> bool {
        (self.start.0 - self.end.0).abs() < 1e-10 && (self.start.1 - self.end.1).abs() < 1e-10
    }

    pub fn axis(&self) -> (f64, f64, f64) {
        (0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conic_arc_creation() {
        let ca = IgesGeomConicArc::new();
        assert_eq!(ca.z_plane(), 0.0);
        assert_eq!(ca.axis(), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_computed_form_number_ellipse() {
        let mut ca = IgesGeomConicArc::new();
        ca.init(1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, (1.0, 0.0), (0.0, 1.0));
        assert_eq!(ca.computed_form_number(), 1);
        assert!(ca.is_from_ellipse());
    }

    #[test]
    fn test_computed_form_number_hyperbola() {
        let mut ca = IgesGeomConicArc::new();
        ca.init(1.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, (1.0, 0.0), (0.0, 1.0));
        assert_eq!(ca.computed_form_number(), 2);
        assert!(ca.is_from_hyperbola());
    }

    #[test]
    fn test_computed_form_number_parabola() {
        let mut ca = IgesGeomConicArc::new();
        ca.init(1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, (0.0, 0.0), (1.0, 1.0));
        assert_eq!(ca.computed_form_number(), 3);
        assert!(ca.is_from_parabola());
    }

    #[test]
    fn test_is_closed() {
        let mut ca = IgesGeomConicArc::new();
        ca.init(1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, (1.0, 1.0), (1.0, 1.0));
        assert!(ca.is_closed());
    }
}
