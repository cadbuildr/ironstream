// FILE: iges_to_b_rep_algo_container.rs
// occt: IGESToBRep_AlgoContainer

#[derive(Default, Clone, Debug)]
pub struct IgesToBRepAlgoContainer;

impl IgesToBRepAlgoContainer {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _container = IgesToBRepAlgoContainer::new();
    }
}
