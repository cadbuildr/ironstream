// FILE: iges_graph_drawing_size.rs
// occt-ref: IGESGraph_Udrawingsize

pub struct UdrawingUsize {
    entity_type: i32,
}

impl UdrawingUsize {
    pub fn new() -> Self {
        UdrawingUsize { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UdrawingUsize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UdrawingUsize::new();
    }
}
