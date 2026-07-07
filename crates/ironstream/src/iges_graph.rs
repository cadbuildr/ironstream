// FILE: iges_graph.rs
// occt: IGESGraph

pub struct Graph;

impl Graph {
    pub fn init() {}
    
    pub fn protocol() -> String {
        "IGESGraph_Protocol".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol() {
        let proto = Graph::protocol();
        assert_eq!(proto, "IGESGraph_Protocol");
    }
}
