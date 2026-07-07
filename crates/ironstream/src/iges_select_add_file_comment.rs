// FILE: iges_select_add_file_comment.rs
// occt: IGESSelect_AddFileComment

pub struct IGESSelectAddFileComment;

impl IGESSelectAddFileComment {
    pub fn new() -> Self {
        IGESSelectAddFileComment
    }
}

impl Default for IGESSelectAddFileComment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectAddFileComment::new();
    }
}
