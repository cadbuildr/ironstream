// FILE: graphic3d_type_of_limit.rs
// occt: Graphic3d_TypeOfLimit

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfLimit {
    MaxNbLights = 0, MaxNbClipPlanes = 1, MaxNbViews = 2, MaxTextureSize = 3,
    MaxViewDumpSizeX = 4, MaxViewDumpSizeY = 5, MaxCombinedTextureUnits = 6,
    MaxMsaa = 7, HasPbr = 8, HasRayTracing = 9, HasRayTracingTextures = 10,
    HasRayTracingAdaptiveSampling = 11, HasRayTracingAdaptiveSamplingAtomic = 12,
    HasSrgb = 13, HasBlendedOit = 14, HasBlendedOitMsaa = 15, HasFlatShading = 16,
    HasMeshEdges = 17, IsWorkaroundFbo = 18, Nb = 19,
}

impl TypeOfLimit {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfLimit::MaxNbLights), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_nb_lights() { assert_eq!(TypeOfLimit::MaxNbLights as u32, 0); }
}
