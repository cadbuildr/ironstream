// FILE: if_select_share_out_result.rs
// occt: IFSelect_ShareOutResult

/// Result of applying a ShareOut to a model or graph.
/// Computes the distribution of entities to output files.
#[derive(Clone, Debug)]
pub struct IFSelectShareOutResult {
    share_out_id: Option<usize>,
    model_id: Option<usize>,
}

impl IFSelectShareOutResult {
    /// Creates a result from a ShareOut and a model
    pub fn from_share_out(share_out_id: usize, model_id: usize) -> Self {
        Self {
            share_out_id: Some(share_out_id),
            model_id: Some(model_id),
        }
    }

    /// Creates a result from a single dispatch and a model
    pub fn from_dispatch(dispatch_id: usize, model_id: usize) -> Self {
        Self {
            share_out_id: None, // Single dispatch, no share out
            model_id: Some(model_id),
        }
    }

    /// Returns the ShareOut ID if created from ShareOut
    pub fn share_out_id(&self) -> Option<usize> {
        self.share_out_id
    }

    /// Returns the model ID
    pub fn model_id(&self) -> Option<usize> {
        self.model_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_share_out() {
        let result = IFSelectShareOutResult::from_share_out(1, 10);
        assert_eq!(result.share_out_id(), Some(1));
        assert_eq!(result.model_id(), Some(10));
    }

    #[test]
    fn test_from_dispatch() {
        let result = IFSelectShareOutResult::from_dispatch(5, 20);
        assert_eq!(result.share_out_id(), None);
        assert_eq!(result.model_id(), Some(20));
    }
}
