// FILE: t_doc_std.rs
// occt: TDocStd

/// The TDocStd package contains main CAF (Computer Aided Framework) classes.
/// This namespace provides utilities for managing standard documents and attributes.
pub struct TDocStd;

impl TDocStd {
    /// Get the list of IDs for TDocStd package attributes.
    /// Returns a list of GUIDs for all standard attributes.
    pub fn id_list() -> Vec<[u8; 16]> {
        vec![
            Self::application_delta_id(),
            Self::compound_delta_id(),
            Self::context_id(),
            Self::format_version_id(),
            Self::multi_transaction_manager_id(),
            Self::owner_id(),
            Self::path_parser_id(),
            Self::x_link_id(),
            Self::x_link_iterator_id(),
            Self::x_link_root_id(),
            Self::x_link_tool_id(),
        ]
    }

    /// Get GUID for ApplicationDelta.
    pub fn application_delta_id() -> [u8; 16] {
        [
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            0xFF, 0x00,
        ]
    }

    /// Get GUID for CompoundDelta.
    pub fn compound_delta_id() -> [u8; 16] {
        [
            0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
            0x00, 0x11,
        ]
    }

    /// Get GUID for Context.
    pub fn context_id() -> [u8; 16] {
        [
            0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00,
            0x11, 0x22,
        ]
    }

    /// Get GUID for FormatVersion.
    pub fn format_version_id() -> [u8; 16] {
        [
            0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11,
            0x22, 0x33,
        ]
    }

    /// Get GUID for MultiTransactionManager.
    pub fn multi_transaction_manager_id() -> [u8; 16] {
        [
            0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22,
            0x33, 0x44,
        ]
    }

    /// Get GUID for Owner.
    pub fn owner_id() -> [u8; 16] {
        [
            0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33,
            0x44, 0x55,
        ]
    }

    /// Get GUID for PathParser.
    pub fn path_parser_id() -> [u8; 16] {
        [
            0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44,
            0x55, 0x66,
        ]
    }

    /// Get GUID for XLink.
    pub fn x_link_id() -> [u8; 16] {
        [
            0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77,
        ]
    }

    /// Get GUID for XLinkIterator.
    pub fn x_link_iterator_id() -> [u8; 16] {
        [
            0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88,
        ]
    }

    /// Get GUID for XLinkRoot.
    pub fn x_link_root_id() -> [u8; 16] {
        [
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99,
        ]
    }

    /// Get GUID for XLinkTool.
    pub fn x_link_tool_id() -> [u8; 16] {
        [
            0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            0x99, 0xAA,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_list_not_empty() {
        let ids = TDocStd::id_list();
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_application_delta_id() {
        let id = TDocStd::application_delta_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_context_id() {
        let id = TDocStd::context_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_owner_id() {
        let id = TDocStd::owner_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_x_link_id() {
        let id = TDocStd::x_link_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_all_ids_are_unique() {
        let ids = TDocStd::id_list();
        let unique_ids: std::collections::HashSet<_> = ids.iter().cloned().collect();
        assert_eq!(ids.len(), unique_ids.len());
    }
}
