// FILE: iges_graph_nominal_size.rs
// occt-ref: IGESGraph_Unominalsize

pub struct UnominalUsize {
    entity_type: i32,
}

impl UnominalUsize {
    pub fn new() -> Self {
        UnominalUsize { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UnominalUsize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UnominalUsize::new();
    }
}
