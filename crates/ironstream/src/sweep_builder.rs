// FILE: sweep_builder.rs
// occt: BRepSweep_Builder, BRepSweep_Sector, BRepFill_Sweep,
//       BRepBuilderAPI_Sweep

/// Sweep mode: how to translate/rotate the profile.
/// occt: BRepSweep_NumShapesMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SweepMode {
    #[default]
    KeepContact,
    SmallSolids,
    FreeBoundary,
}

impl SweepMode {
    pub fn is_contact_required(&self) -> bool {
        matches!(self, Self::KeepContact)
    }
}

/// Sweep sector: profile + spine segment.
/// occt: BRepSweep_Sector
#[derive(Clone, Debug)]
pub struct SweepSector {
    pub sector_id: u32,
    pub profile_id: u32,
    pub spine_start: u32,
    pub spine_end: u32,
    pub is_built: bool,
}

impl SweepSector {
    pub fn new(sid: u32, prof: u32) -> Self {
        Self {
            sector_id: sid,
            profile_id: prof,
            spine_start: 0,
            spine_end: 0,
            is_built: false,
        }
    }

    pub fn set_spine_interval(&mut self, start: u32, end: u32) {
        self.spine_start = start;
        self.spine_end = end;
    }

    pub fn build(&mut self) { self.is_built = true; }
    pub fn is_valid(&self) -> bool { self.is_built && self.spine_end > self.spine_start }
}

/// Sweep builder: sweeps a profile along a spine.
/// occt: BRepSweep_Builder / BRepFill_Sweep
#[derive(Clone, Debug)]
pub struct SweepBuilder {
    pub profile_id: u32,
    pub spine_id: u32,
    pub mode: SweepMode,
    pub sectors: Vec<SweepSector>,
    pub keep_contact: bool,
    pub sweep_result_id: Option<u32>,
}

impl SweepBuilder {
    pub fn new(prof_id: u32, spine_id: u32) -> Self {
        Self {
            profile_id: prof_id,
            spine_id,
            mode: SweepMode::KeepContact,
            sectors: Vec::new(),
            keep_contact: true,
            sweep_result_id: None,
        }
    }

    pub fn set_mode(&mut self, m: SweepMode) { self.mode = m; }

    pub fn add_sector(&mut self, sector: SweepSector) {
        self.sectors.push(sector);
    }

    pub fn nb_sectors(&self) -> usize { self.sectors.len() }

    pub fn perform(&mut self) -> bool {
        if self.sectors.is_empty() { return false; }
        for sector in &mut self.sectors { sector.build(); }
        let result_id = (self.profile_id + self.spine_id) * 100 + self.sectors.len() as u32;
        self.sweep_result_id = Some(result_id);
        true
    }

    pub fn result(&self) -> Option<u32> { self.sweep_result_id }
    pub fn is_done(&self) -> bool { self.sweep_result_id.is_some() }
}

/// Sweep with scaling: profile scales as it sweeps.
/// occt: BRepBuilderAPI_Sweep with scale function
#[derive(Clone, Debug)]
pub struct SweepWithScale {
    pub base_sweep: SweepBuilder,
    pub scale_function: Vec<f64>,  // scale factors at spine parameters
}

impl SweepWithScale {
    pub fn new(prof_id: u32, spine_id: u32) -> Self {
        Self {
            base_sweep: SweepBuilder::new(prof_id, spine_id),
            scale_function: vec![1.0],
        }
    }

    pub fn set_scale_function(&mut self, scales: Vec<f64>) {
        self.scale_function = scales.iter().map(|s| s.max(0.1)).collect();
    }

    pub fn perform(&mut self) -> bool {
        if self.scale_function.is_empty() {
            return false;
        }
        // occt: BRepFill_Sweep::Build generates the intermediate sections
        // internally from the section (scale) law — the caller never supplies
        // them. Mirror that: when no sectors were added explicitly, derive one
        // sector per scale-function interval along the spine.
        if self.base_sweep.sectors.is_empty() {
            let nb_samples = self.scale_function.len().max(2);
            for i in 0..(nb_samples - 1) {
                let mut sector =
                    SweepSector::new(i as u32 + 1, self.base_sweep.profile_id);
                sector.set_spine_interval(i as u32, i as u32 + 1);
                self.base_sweep.add_sector(sector);
            }
        }
        self.base_sweep.perform()
    }

    pub fn result(&self) -> Option<u32> { self.base_sweep.result() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_mode() {
        assert!(SweepMode::KeepContact.is_contact_required());
        assert!(!SweepMode::FreeBoundary.is_contact_required());
    }

    #[test]
    fn sweep_sector() {
        let mut s = SweepSector::new(1, 10);
        s.set_spine_interval(0, 5);
        assert!(!s.is_valid()); // not built yet
        s.build();
        assert!(s.is_valid());
    }

    #[test]
    fn sweep_builder() {
        let mut sb = SweepBuilder::new(5, 3);
        sb.add_sector(SweepSector::new(1, 5));
        assert_eq!(sb.nb_sectors(), 1);
        assert!(sb.perform());
        assert!(sb.is_done());
        assert!(sb.result().is_some());
    }

    #[test]
    fn sweep_with_scale() {
        let mut sws = SweepWithScale::new(2, 4);
        sws.set_scale_function(vec![1.0, 0.8, 0.6]);
        assert_eq!(sws.scale_function.len(), 3);
        assert!(sws.perform());
    }
}
