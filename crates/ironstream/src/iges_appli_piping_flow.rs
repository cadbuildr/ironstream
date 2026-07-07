// FILE: iges_appli_piping_flow.rs
// occt: IGESAppli_PipingFlow

/// Represents piping flow network properties.
#[derive(Clone, Debug)]
pub struct IgesAppliPipingFlow {
    source_node: i32,
    dest_node: i32,
    flow_direction: i32,
}

impl IgesAppliPipingFlow {
    pub fn new() -> Self {
        Self {
            source_node: 0,
            dest_node: 0,
            flow_direction: 0,
        }
    }

    pub fn init(&mut self, src: i32, dst: i32, dir: i32) {
        self.source_node = src;
        self.dest_node = dst;
        self.flow_direction = dir;
    }

    pub fn source_node(&self) -> i32 {
        self.source_node
    }

    pub fn dest_node(&self) -> i32 {
        self.dest_node
    }

    pub fn flow_direction(&self) -> i32 {
        self.flow_direction
    }
}

impl Default for IgesAppliPipingFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut flow = IgesAppliPipingFlow::new();
        flow.init(10, 20, 1);

        assert_eq!(flow.source_node(), 10);
        assert_eq!(flow.dest_node(), 20);
        assert_eq!(flow.flow_direction(), 1);
    }
}
