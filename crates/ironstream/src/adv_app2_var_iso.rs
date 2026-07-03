// FILE: adv_app2_var_iso.rs
// occt: AdvApp2Var_Iso

//! Used to store constraints on a line U = Ui or V = Vj

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoType {
    IsoU,
    IsoV,
}

pub struct Iso {
    pub iso_type: IsoType,
    pub constant_param: f64,
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
    pub position: i32,
    pub u_order: i32,
    pub v_order: i32,
    pub approximated: bool,
    pub has_result: bool,
    pub coefficients: Vec<f64>,
}

impl Iso {
    pub fn new_u(u_const: f64) -> Self {
        Iso {
            iso_type: IsoType::IsoU,
            constant_param: u_const,
            u_first: 0.0,
            u_last: 1.0,
            v_first: 0.0,
            v_last: 1.0,
            position: 0,
            u_order: 3,
            v_order: 3,
            approximated: false,
            has_result: false,
            coefficients: vec![],
        }
    }

    pub fn new_v(v_const: f64) -> Self {
        Iso {
            iso_type: IsoType::IsoV,
            constant_param: v_const,
            u_first: 0.0,
            u_last: 1.0,
            v_first: 0.0,
            v_last: 1.0,
            position: 0,
            u_order: 3,
            v_order: 3,
            approximated: false,
            has_result: false,
            coefficients: vec![],
        }
    }

    pub fn with_domain(
        iso_type: IsoType,
        constant: f64,
        u_first: f64,
        u_last: f64,
        v_first: f64,
        v_last: f64,
        position: i32,
    ) -> Self {
        Iso {
            iso_type,
            constant_param: constant,
            u_first,
            u_last,
            v_first,
            v_last,
            position,
            u_order: 3,
            v_order: 3,
            approximated: false,
            has_result: false,
            coefficients: vec![],
        }
    }

    pub fn is_approximated(&self) -> bool {
        self.approximated
    }

    pub fn has_result(&self) -> bool {
        self.has_result
    }

    pub fn mark_approximated(&mut self) {
        self.approximated = true;
    }

    pub fn set_coefficients(&mut self, coeff: Vec<f64>) {
        self.coefficients = coeff;
        self.has_result = !self.coefficients.is_empty();
    }

    pub fn change_domain(&mut self, u_first: f64, u_last: f64) {
        self.u_first = u_first;
        self.u_last = u_last;
    }

    pub fn change_domain_full(
        &mut self,
        u_first: f64,
        u_last: f64,
        v_first: f64,
        v_last: f64,
    ) {
        self.u_first = u_first;
        self.u_last = u_last;
        self.v_first = v_first;
        self.v_last = v_last;
    }

    pub fn reset_approx(&mut self) {
        self.approximated = false;
        self.has_result = false;
        self.coefficients.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_u_iso() {
        let iso = Iso::new_u(0.5);
        assert_eq!(iso.iso_type, IsoType::IsoU);
        assert_eq!(iso.constant_param, 0.5);
        assert!(!iso.approximated);
    }

    #[test]
    fn test_new_v_iso() {
        let iso = Iso::new_v(0.3);
        assert_eq!(iso.iso_type, IsoType::IsoV);
        assert_eq!(iso.constant_param, 0.3);
    }

    #[test]
    fn test_with_domain() {
        let iso = Iso::with_domain(
            IsoType::IsoU,
            0.5,
            0.0,
            1.0,
            0.0,
            1.0,
            1,
        );
        assert_eq!(iso.u_first, 0.0);
        assert_eq!(iso.u_last, 1.0);
        assert_eq!(iso.position, 1);
    }

    #[test]
    fn test_mark_approximated() {
        let mut iso = Iso::new_u(0.5);
        iso.mark_approximated();
        assert!(iso.is_approximated());
    }

    #[test]
    fn test_set_coefficients() {
        let mut iso = Iso::new_u(0.5);
        iso.set_coefficients(vec![1.0, 2.0, 3.0]);
        assert!(iso.has_result());
        assert_eq!(iso.coefficients.len(), 3);
    }

    #[test]
    fn test_change_domain() {
        let mut iso = Iso::new_u(0.5);
        iso.change_domain(0.1, 0.9);
        assert_eq!(iso.u_first, 0.1);
        assert_eq!(iso.u_last, 0.9);
    }

    #[test]
    fn test_reset_approx() {
        let mut iso = Iso::new_u(0.5);
        iso.set_coefficients(vec![1.0, 2.0]);
        iso.mark_approximated();
        iso.reset_approx();
        assert!(!iso.approximated);
        assert!(!iso.has_result());
    }
}
