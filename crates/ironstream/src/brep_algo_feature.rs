// FILE: rust/ironstream/crates/ironstream/src/brep_algo_feature.rs

// occt-note: feature type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlgoFeatureType {
    Hole,
    Pocket,
    Slot,
    Protrusion,
    Unknown,
}

// occt-note: detected feature
#[derive(Clone, Debug)]
pub struct AlgoFeature {
    feature_type: AlgoFeatureType,
    center: [f64; 3],
    size: [f64; 3],
    nb_faces: usize,
}

impl AlgoFeature {
    pub fn new(
        feature_type: AlgoFeatureType,
        center: [f64; 3],
        size: [f64; 3],
        nb_faces: usize,
    ) -> Self {
        Self {
            feature_type,
            center,
            size,
            nb_faces,
        }
    }

    pub fn feature_type(&self) -> AlgoFeatureType {
        self.feature_type
    }

    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    pub fn size(&self) -> [f64; 3] {
        self.size
    }

    pub fn nb_faces(&self) -> usize {
        self.nb_faces
    }

    pub fn volume(&self) -> f64 {
        self.size[0] * self.size[1] * self.size[2]
    }
}

// occt-note: BRepAlgoAPI feature operation result
#[derive(Debug)]
pub struct BrepAlgoFeatureResult {
    pub is_done: bool,
    pub nb_faces_modified: usize,
    pub features: Vec<AlgoFeature>,
}

impl BrepAlgoFeatureResult {
    pub fn new() -> Self {
        Self {
            is_done: false,
            nb_faces_modified: 0,
            features: Vec::new(),
        }
    }

    pub fn mark_done(&mut self, nb_faces: usize) {
        self.is_done = true;
        self.nb_faces_modified = nb_faces;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_faces_modified(&self) -> usize {
        self.nb_faces_modified
    }

    pub fn nb_features(&self) -> usize {
        self.features.len()
    }

    pub fn feature(&self, i: usize) -> &AlgoFeature {
        &self.features[i]
    }

    pub fn add_feature(&mut self, f: AlgoFeature) {
        self.features.push(f);
    }
}

impl Default for BrepAlgoFeatureResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: BRepAlgoAPI_RemoveFeatures stub
pub struct BrepAlgoFeatureRemoval {
    pub features_to_remove: Vec<String>,
    pub result: BrepAlgoFeatureResult,
}

impl BrepAlgoFeatureRemoval {
    pub fn new() -> Self {
        Self {
            features_to_remove: Vec::new(),
            result: BrepAlgoFeatureResult::new(),
        }
    }

    pub fn add_feature_to_remove(&mut self, label: &str) {
        self.features_to_remove.push(label.to_string());
    }

    pub fn nb_to_remove(&self) -> usize {
        self.features_to_remove.len()
    }

    pub fn perform(&mut self) {
        let nb = self.nb_to_remove();
        self.result.mark_done(nb);
    }

    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    pub fn result(&self) -> &BrepAlgoFeatureResult {
        &self.result
    }
}

impl Default for BrepAlgoFeatureRemoval {
    fn default() -> Self {
        Self::new()
    }
}
