// FILE: hlr_brep.rs
// occt: HLRBRep // — Hidden Line Removal for BRep shapes.

// occt: HLRBRep_TypeOfResultingEdge
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HlrBrepEdgeType {
    VisibleSharp,
    VisibleSmooth,
    VisibleSewing,
    VisibleOutline,
    HiddenSharp,
    HiddenSmooth,
    HiddenSewing,
    HiddenOutline,
}

impl HlrBrepEdgeType {
    pub fn is_visible(self) -> bool {
        matches!(
            self,
            HlrBrepEdgeType::VisibleSharp
                | HlrBrepEdgeType::VisibleSmooth
                | HlrBrepEdgeType::VisibleSewing
                | HlrBrepEdgeType::VisibleOutline
        )
    }

    pub fn is_hidden(self) -> bool {
        !self.is_visible()
    }

    pub fn is_outline(self) -> bool {
        matches!(
            self,
            HlrBrepEdgeType::VisibleOutline | HlrBrepEdgeType::HiddenOutline
        )
    }
}

// occt: HLRBRep_EdgeData
pub struct HlrBrepEdgeData {
    pub edge_type: HlrBrepEdgeType,
    pub start_param: f64,
    pub end_param: f64,
}

impl HlrBrepEdgeData {
    pub fn new(edge_type: HlrBrepEdgeType, start: f64, end: f64) -> Self {
        Self {
            edge_type,
            start_param: start,
            end_param: end,
        }
    }
}

// occt: HLRBRep_Algo
pub struct HlrBrepAlgo {
    edges: Vec<HlrBrepEdgeData>,
    up_to_date: bool,
}

impl HlrBrepAlgo {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            up_to_date: false,
        }
    }

    pub fn add_edge(&mut self, e: HlrBrepEdgeData) {
        self.up_to_date = false;
        self.edges.push(e);
    }

    pub fn update(&mut self) {
        self.up_to_date = true;
    }

    pub fn is_up_to_date(&self) -> bool {
        self.up_to_date
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub(crate) fn edges(&self) -> &[HlrBrepEdgeData] {
        &self.edges
    }
}

impl Default for HlrBrepAlgo {
    fn default() -> Self {
        Self::new()
    }
}

// occt: HLRBRep_HLRToShape
pub struct HlrBrepHLRToShape<'a> {
    algo: &'a HlrBrepAlgo,
}

impl<'a> HlrBrepHLRToShape<'a> {
    pub fn new(algo: &'a HlrBrepAlgo) -> Self {
        Self { algo }
    }

    pub fn visible_edges(&self) -> Vec<&HlrBrepEdgeData> {
        self.algo
            .edges()
            .iter()
            .filter(|e| e.edge_type.is_visible())
            .collect()
    }

    pub fn hidden_edges(&self) -> Vec<&HlrBrepEdgeData> {
        self.algo
            .edges()
            .iter()
            .filter(|e| e.edge_type.is_hidden())
            .collect()
    }

    pub fn visible_outline_edges(&self) -> Vec<&HlrBrepEdgeData> {
        self.algo
            .edges()
            .iter()
            .filter(|e| e.edge_type == HlrBrepEdgeType::VisibleOutline)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_new_not_up_to_date() {
        let algo = HlrBrepAlgo::new();
        assert!(!algo.is_up_to_date());
        assert_eq!(algo.edge_count(), 0);
    }

    #[test]
    fn algo_update_sets_up_to_date() {
        let mut algo = HlrBrepAlgo::new();
        algo.update();
        assert!(algo.is_up_to_date());
    }

    #[test]
    fn add_edge_clears_up_to_date() {
        let mut algo = HlrBrepAlgo::new();
        algo.update();
        assert!(algo.is_up_to_date());
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
        assert!(!algo.is_up_to_date());
        assert_eq!(algo.edge_count(), 1);
    }

    #[test]
    fn edge_type_visible_predicate() {
        assert!(HlrBrepEdgeType::VisibleSharp.is_visible());
        assert!(HlrBrepEdgeType::VisibleSmooth.is_visible());
        assert!(HlrBrepEdgeType::VisibleSewing.is_visible());
        assert!(HlrBrepEdgeType::VisibleOutline.is_visible());
        assert!(!HlrBrepEdgeType::HiddenSharp.is_visible());
        assert!(!HlrBrepEdgeType::HiddenSmooth.is_visible());
        assert!(!HlrBrepEdgeType::HiddenSewing.is_visible());
        assert!(!HlrBrepEdgeType::HiddenOutline.is_visible());
    }

    #[test]
    fn edge_type_hidden_predicate() {
        assert!(!HlrBrepEdgeType::VisibleSharp.is_hidden());
        assert!(HlrBrepEdgeType::HiddenSharp.is_hidden());
    }

    #[test]
    fn edge_type_outline_predicate() {
        assert!(HlrBrepEdgeType::VisibleOutline.is_outline());
        assert!(HlrBrepEdgeType::HiddenOutline.is_outline());
        assert!(!HlrBrepEdgeType::VisibleSharp.is_outline());
        assert!(!HlrBrepEdgeType::HiddenSharp.is_outline());
    }

    #[test]
    fn hlr_to_shape_filters_visible() {
        let mut algo = HlrBrepAlgo::new();
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSharp, 0.0, 1.0));
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleOutline, 0.0, 1.0));
        let extractor = HlrBrepHLRToShape::new(&algo);
        let visible = extractor.visible_edges();
        assert_eq!(visible.len(), 2);
    }

    #[test]
    fn hlr_to_shape_filters_hidden() {
        let mut algo = HlrBrepAlgo::new();
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSharp, 0.0, 1.0));
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSmooth, 0.0, 1.0));
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
        let extractor = HlrBrepHLRToShape::new(&algo);
        let hidden = extractor.hidden_edges();
        assert_eq!(hidden.len(), 2);
    }

    #[test]
    fn hlr_to_shape_visible_outline_only() {
        let mut algo = HlrBrepAlgo::new();
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleOutline, 0.0, 1.0));
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenOutline, 0.0, 1.0));
        algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
        let extractor = HlrBrepHLRToShape::new(&algo);
        let outlines = extractor.visible_outline_edges();
        assert_eq!(outlines.len(), 1);
        assert_eq!(outlines[0].edge_type, HlrBrepEdgeType::VisibleOutline);
    }

    #[test]
    fn edge_data_stores_params() {
        let e = HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSmooth, 0.25, 0.75);
        assert_eq!(e.edge_type, HlrBrepEdgeType::VisibleSmooth);
        assert!((e.start_param - 0.25).abs() < 1e-15);
        assert!((e.end_param - 0.75).abs() < 1e-15);
    }
}
