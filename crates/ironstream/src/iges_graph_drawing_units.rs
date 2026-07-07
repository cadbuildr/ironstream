// FILE: iges_graph_drawing_units.rs
// occt: IGESGraph_Udrawingunits

pub struct UdrawingUunits {
    entity_type: i32,
}

impl UdrawingUunits {
    pub fn new() -> Self {
        UdrawingUunits { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UdrawingUunits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UdrawingUunits::new();
    }
}
