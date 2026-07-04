// FILE: if_select_transformer.rs
// occt: IFSelect_Transformer

/// Base class for transformers that modify entities.
#[derive(Clone, Debug)]
pub struct IFSelectTransformer {
    transformer_id: String,
}

impl IFSelectTransformer {
    /// Creates a Transformer
    pub fn new(id: String) -> Self {
        Self {
            transformer_id: id,
        }
    }

    /// Returns the transformer ID
    pub fn id(&self) -> &str {
        &self.transformer_id
    }

    /// Returns a label describing the transformer
    pub fn label(&self) -> String {
        format!("Transformer: {}", self.transformer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let transformer = IFSelectTransformer::new("trans1".to_string());
        assert_eq!(transformer.id(), "trans1");
    }

    #[test]
    fn test_label() {
        let transformer = IFSelectTransformer::new("test".to_string());
        assert_eq!(transformer.label(), "Transformer: test");
    }
}
