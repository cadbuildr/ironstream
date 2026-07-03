// FILE: adv_app2_var_context.rs
// occt: AdvApp2Var_Context

//! Contains all parameters for surface approximation
//! (tolerance, computing options, etc.)

pub struct Context {
    pub favor_iso: i32,
    pub u_order: i32,
    pub v_order: i32,
    pub u_limit: i32,
    pub v_limit: i32,
    pub nb_1d_ss: i32,
    pub nb_2d_ss: i32,
    pub nb_3d_ss: i32,
    pub precision: i32,
}

impl Context {
    pub fn new() -> Self {
        Context {
            favor_iso: 0,
            u_order: 3,
            v_order: 3,
            u_limit: 16,
            v_limit: 16,
            nb_1d_ss: 0,
            nb_2d_ss: 0,
            nb_3d_ss: 0,
            precision: 0,
        }
    }

    pub fn with_params(
        favor_iso: i32,
        u_order: i32,
        v_order: i32,
        u_limit: i32,
        v_limit: i32,
        nb_1d_ss: i32,
        nb_2d_ss: i32,
        nb_3d_ss: i32,
        precision: i32,
    ) -> Self {
        Context {
            favor_iso,
            u_order,
            v_order,
            u_limit,
            v_limit,
            nb_1d_ss,
            nb_2d_ss,
            nb_3d_ss,
            precision,
        }
    }

    pub fn total_dimension(&self) -> i32 {
        self.nb_1d_ss + self.nb_2d_ss * 2 + self.nb_3d_ss * 3
    }

    pub fn total_number_ssp(&self) -> i32 {
        self.nb_1d_ss + self.nb_2d_ss + self.nb_3d_ss
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_context() {
        let ctx = Context::new();
        assert_eq!(ctx.u_order, 3);
        assert_eq!(ctx.v_order, 3);
    }

    #[test]
    fn test_total_dimension() {
        let ctx = Context::with_params(0, 3, 3, 16, 16, 1, 2, 1, 0);
        assert_eq!(ctx.total_dimension(), 1 + 2 * 2 + 1 * 3);
    }

    #[test]
    fn test_total_number_ssp() {
        let ctx = Context::with_params(0, 3, 3, 16, 16, 2, 3, 4, 0);
        assert_eq!(ctx.total_number_ssp(), 2 + 3 + 4);
    }
}
