// FILE: iges_graph_protocol.rs
// occt: IGESGraph_Protocol

pub struct GraphProtocol {
    nb_resources: i32,
}

impl GraphProtocol {
    pub fn new() -> Self {
        GraphProtocol { nb_resources: 1 }
    }

    pub fn nb_resources(&self) -> i32 {
        self.nb_resources
    }

    pub fn resource(&self, num: i32) -> Option<i32> {
        if num == 1 {
            Some(1)
        } else {
            None
        }
    }

    pub fn type_number(&self, type_name: &str) -> Option<i32> {
        match type_name {
            "Color" => Some(1),
            "DefinitionLevel" => Some(2),
            "DrawingSize" => Some(3),
            "DrawingUnits" => Some(4),
            "HighLight" => Some(5),
            "IntercharacterSpacing" => Some(6),
            "LineFontDefPattern" => Some(7),
            "LineFontDefTemplate" => Some(8),
            "LineFontPredefined" => Some(9),
            "NominalSize" => Some(10),
            _ => None,
        }
    }
}

impl Default for GraphProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let proto = GraphProtocol::new();
        assert_eq!(proto.nb_resources(), 1);
    }
}
