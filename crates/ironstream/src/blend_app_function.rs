// FILE: blend_app_function.rs
// occt: Blend_AppFunction

#[derive(Clone)]
pub struct BlendAppFunction {
    nb_variables: i32,
    nb_equations: i32,
}

impl BlendAppFunction {
    pub fn new() -> Self {
        Self {
            nb_variables: 4,
            nb_equations: 4,
        }
    }

    pub fn set_nb_variables(&mut self, n: i32) {
        self.nb_variables = n;
    }

    pub fn set_nb_equations(&mut self, n: i32) {
        self.nb_equations = n;
    }

    pub fn nb_variables(&self) -> i32 {
        self.nb_variables
    }

    pub fn nb_equations(&self) -> i32 {
        self.nb_equations
    }

    pub fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool {
        if x.len() < self.nb_variables as usize || f.len() < self.nb_equations as usize {
            return false;
        }
        for v in f.iter_mut() {
            *v = 0.0;
        }
        true
    }

    pub fn derivatives(&self, x: &[f64], d: &mut [[f64; 4]; 4]) -> bool {
        if x.len() < self.nb_variables as usize {
            return false;
        }
        for i in 0..4 {
            for j in 0..4 {
                d[i][j] = 0.0;
            }
        }
        true
    }
}

impl Default for BlendAppFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let app = BlendAppFunction::new();
        assert_eq!(app.nb_variables(), 4);
        assert_eq!(app.nb_equations(), 4);
    }

    #[test]
    fn test_set_nb() {
        let mut app = BlendAppFunction::new();
        app.set_nb_variables(5);
        app.set_nb_equations(5);
        assert_eq!(app.nb_variables(), 5);
        assert_eq!(app.nb_equations(), 5);
    }

    #[test]
    fn test_value() {
        let mut app = BlendAppFunction::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(app.value(&x, &mut f));
    }
}
