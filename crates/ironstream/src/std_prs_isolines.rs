// FILE: std_prs_isolines.rs
// occt: StdPrs_Isolines

/// Isoline presentation on surfaces.
#[derive(Clone, Debug)]
pub struct Isolines {
    pub isoline_id: u32,
    pub u_count: usize,
    pub v_count: usize,
}

impl Isolines {
    pub fn new(isoline_id: u32) -> Self {
        Self {
            isoline_id,
            u_count: 0,
            v_count: 0,
        }
    }

    pub fn set_u_isolines(&mut self, count: usize) {
        self.u_count = count;
    }

    pub fn set_v_isolines(&mut self, count: usize) {
        self.v_count = count;
    }

    pub fn nb_u_isolines(&self) -> usize {
        self.u_count
    }

    pub fn nb_v_isolines(&self) -> usize {
        self.v_count
    }

    pub fn total_isolines(&self) -> usize {
        self.u_count + self.v_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_isolines() {
        let iso = Isolines::new(1);
        assert_eq!(iso.isoline_id, 1);
        assert_eq!(iso.total_isolines(), 0);
    }

    #[test]
    fn set_isolines() {
        let mut iso = Isolines::new(1);
        iso.set_u_isolines(10);
        iso.set_v_isolines(20);
        assert_eq!(iso.nb_u_isolines(), 10);
        assert_eq!(iso.nb_v_isolines(), 20);
        assert_eq!(iso.total_isolines(), 30);
    }
}
