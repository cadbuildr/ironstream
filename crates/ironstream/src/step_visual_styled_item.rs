// FILE: step_visual_styled_item.rs
// occt: StepVisual_StyledItem

use std::sync::Arc;

pub struct HasciiString;
pub struct PresentationStyleAssignment;
pub struct RepresentationItem;
pub struct TransientRef;

pub struct StyledItem {
    name: Option<Arc<HasciiString>>,
    styles: Option<Arc<Vec<Arc<PresentationStyleAssignment>>>>,
    item: Option<Arc<TransientRef>>,
    repr_item: Option<Arc<RepresentationItem>>,
}

impl StyledItem {
    pub fn new() -> Self {
        StyledItem {
            name: None,
            styles: None,
            item: None,
            repr_item: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Arc<HasciiString>>,
        styles: Option<Arc<Vec<Arc<PresentationStyleAssignment>>>>,
        item: Option<Arc<TransientRef>>,
    ) {
        self.name = name;
        self.styles = styles;
        self.item = item;
    }

    pub fn set_styles(
        &mut self,
        styles: Option<Arc<Vec<Arc<PresentationStyleAssignment>>>>,
    ) {
        self.styles = styles;
    }

    pub fn styles(&self) -> Option<&Arc<Vec<Arc<PresentationStyleAssignment>>>> {
        self.styles.as_ref()
    }

    pub fn styles_value(&self, num: usize) -> Option<&Arc<PresentationStyleAssignment>> {
        self.styles.as_ref().and_then(|styles| styles.get(num))
    }

    pub fn nb_styles(&self) -> usize {
        self.styles.as_ref().map(|s| s.len()).unwrap_or(0)
    }

    pub fn set_item(&mut self, item: Option<Arc<RepresentationItem>>) {
        if let Some(ref item_ref) = item {
            self.repr_item = Some(item_ref.clone());
        }
    }

    pub fn item(&self) -> Option<&Arc<RepresentationItem>> {
        self.repr_item.as_ref()
    }

    pub fn set_item_target(&mut self, target: Option<StyledItemTarget>) {
        // For AP242 compatibility
        self.item = target.map(|_| Arc::new(TransientRef));
    }

    pub fn item_ap242(&self) -> Option<StyledItemTarget> {
        if self.item.is_some() {
            Some(StyledItemTarget::new())
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<&Arc<HasciiString>> {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: Option<Arc<HasciiString>>) {
        self.name = name;
    }
}

pub struct StyledItemTarget;

impl StyledItemTarget {
    fn new() -> Self {
        StyledItemTarget
    }
}

impl Default for StyledItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let si = StyledItem::new();
        assert_eq!(si.nb_styles(), 0);
        assert!(si.styles().is_none());
        assert!(si.item().is_none());
    }

    #[test]
    fn test_set_and_get_styles() {
        let mut si = StyledItem::new();
        let styles = vec![Arc::new(PresentationStyleAssignment)];
        si.set_styles(Some(Arc::new(styles)));
        assert_eq!(si.nb_styles(), 1);
        assert!(si.styles().is_some());
    }

    #[test]
    fn test_set_and_get_item() {
        let mut si = StyledItem::new();
        let item = Arc::new(RepresentationItem);
        si.set_item(Some(item.clone()));
        assert!(si.item().is_some());
    }

    #[test]
    fn test_init() {
        let mut si = StyledItem::new();
        let styles = vec![Arc::new(PresentationStyleAssignment)];
        let item = Arc::new(TransientRef);
        si.init(None, Some(Arc::new(styles)), Some(item));

        assert_eq!(si.nb_styles(), 1);
    }

    #[test]
    fn test_styles_value() {
        let mut si = StyledItem::new();
        let styles = vec![Arc::new(PresentationStyleAssignment); 3];
        si.set_styles(Some(Arc::new(styles)));
        assert!(si.styles_value(0).is_some());
        assert!(si.styles_value(1).is_some());
        assert!(si.styles_value(2).is_some());
        assert!(si.styles_value(3).is_none());
    }
}
