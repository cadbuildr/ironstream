// FILE: bop_algo_maker_volume.rs
// occt: BOPAlgo_MakerVolume

pub struct BopAlgoMakerVolume;

impl BopAlgoMakerVolume {
    pub fn new() -> Self {
        BopAlgoMakerVolume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoMakerVolume::new();
    }
}
