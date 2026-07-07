// FILE: iges_solid_solid_assembly.rs
// occt: IGESSolid_SolidAssembly

//! Solid Assembly entity (IGES Type 516, Form 1).
//!
//! Represents an assembly of solids.

#[derive(Clone)]
pub struct Solid {
    id: usize,
}

impl Solid {
    pub fn new(id: usize) -> Self {
        Solid { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Solid assembly entity
pub struct IGESSolidSolidAssembly {
    solids: Vec<Solid>,
}

impl IGESSolidSolidAssembly {
    /// Creates a new solid assembly
    pub fn new() -> Self {
        IGESSolidSolidAssembly {
            solids: Vec::new(),
        }
    }

    /// Initializes the assembly with solids
    pub fn init(&mut self, solids: Vec<Solid>) {
        self.solids = solids;
    }

    /// Returns the number of solids
    pub fn nb_solids(&self) -> usize {
        self.solids.len()
    }

    /// Returns the index-th solid
    pub fn solid(&self, index: usize) -> Option<&Solid> {
        self.solids.get(index)
    }

    /// Returns all solids
    pub fn solids(&self) -> &[Solid] {
        &self.solids
    }

    /// Adds a solid to the assembly
    pub fn add_solid(&mut self, solid: Solid) {
        self.solids.push(solid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_creation() {
        let s = Solid::new(1);
        assert_eq!(s.id(), 1);
        assert!(!s.is_null());
    }

    #[test]
    fn test_assembly_creation() {
        let a = IGESSolidSolidAssembly::new();
        assert_eq!(a.nb_solids(), 0);
    }

    #[test]
    fn test_assembly_init() {
        let mut a = IGESSolidSolidAssembly::new();
        let solids = vec![Solid::new(1), Solid::new(2), Solid::new(3)];

        a.init(solids);

        assert_eq!(a.nb_solids(), 3);
    }

    #[test]
    fn test_assembly_solid() {
        let mut a = IGESSolidSolidAssembly::new();
        let solids = vec![Solid::new(10), Solid::new(20)];

        a.init(solids);

        assert_eq!(a.solid(0).unwrap().id(), 10);
        assert_eq!(a.solid(1).unwrap().id(), 20);
        assert!(a.solid(2).is_none());
    }

    #[test]
    fn test_assembly_add_solid() {
        let mut a = IGESSolidSolidAssembly::new();
        a.add_solid(Solid::new(1));
        a.add_solid(Solid::new(2));

        assert_eq!(a.nb_solids(), 2);
    }
}
