// FILE: bvh_radix_sorter.rs

// occt: BVH_RadixSorter

/// Pair of Morton code and primitive ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncodedLink {
    pub code: u32,
    pub id: usize,
}

impl EncodedLink {
    pub fn new(code: u32, id: usize) -> Self {
        EncodedLink { code, id }
    }
}

/// Lookup table for expanding 8-bit value to 24-bit Morton code component.
/// Each bit is spread to every 3rd position for interleaving with other components.
const MORTON_LUT: [u32; 256] = [
    0x000000, 0x000001, 0x000008, 0x000009, 0x000040, 0x000041, 0x000048, 0x000049, 0x000200,
    0x000201, 0x000208, 0x000209, 0x000240, 0x000241, 0x000248, 0x000249, 0x001000, 0x001001,
    0x001008, 0x001009, 0x001040, 0x001041, 0x001048, 0x001049, 0x001200, 0x001201, 0x001208,
    0x001209, 0x001240, 0x001241, 0x001248, 0x001249, 0x008000, 0x008001, 0x008008, 0x008009,
    0x008040, 0x008041, 0x008048, 0x008049, 0x008200, 0x008201, 0x008208, 0x008209, 0x008240,
    0x008241, 0x008248, 0x008249, 0x009000, 0x009001, 0x009008, 0x009009, 0x009040, 0x009041,
    0x009048, 0x009049, 0x009200, 0x009201, 0x009208, 0x009209, 0x009240, 0x009241, 0x009248,
    0x009249, 0x040000, 0x040001, 0x040008, 0x040009, 0x040040, 0x040041, 0x040048, 0x040049,
    0x040200, 0x040201, 0x040208, 0x040209, 0x040240, 0x040241, 0x040248, 0x040249, 0x041000,
    0x041001, 0x041008, 0x041009, 0x041040, 0x041041, 0x041048, 0x041049, 0x041200, 0x041201,
    0x041208, 0x041209, 0x041240, 0x041241, 0x041248, 0x041249, 0x048000, 0x048001, 0x048008,
    0x048009, 0x048040, 0x048041, 0x048048, 0x048049, 0x048200, 0x048201, 0x048208, 0x048209,
    0x048240, 0x048241, 0x048248, 0x048249, 0x049000, 0x049001, 0x049008, 0x049009, 0x049040,
    0x049041, 0x049048, 0x049049, 0x049200, 0x049201, 0x049208, 0x049209, 0x049240, 0x049241,
    0x049248, 0x049249, 0x200000, 0x200001, 0x200008, 0x200009, 0x200040, 0x200041, 0x200048,
    0x200049, 0x200200, 0x200201, 0x200208, 0x200209, 0x200240, 0x200241, 0x200248, 0x200249,
    0x201000, 0x201001, 0x201008, 0x201009, 0x201040, 0x201041, 0x201048, 0x201049, 0x201200,
    0x201201, 0x201208, 0x201209, 0x201240, 0x201241, 0x201248, 0x201249, 0x208000, 0x208001,
    0x208008, 0x208009, 0x208040, 0x208041, 0x208048, 0x208049, 0x208200, 0x208201, 0x208208,
    0x208209, 0x208240, 0x208241, 0x208248, 0x208249, 0x209000, 0x209001, 0x209008, 0x209009,
    0x209040, 0x209041, 0x209048, 0x209049, 0x209200, 0x209201, 0x209208, 0x209209, 0x209240,
    0x209241, 0x209248, 0x209249, 0x240000, 0x240001, 0x240008, 0x240009, 0x240040, 0x240041,
    0x240048, 0x240049, 0x240200, 0x240201, 0x240208, 0x240209, 0x240240, 0x240241, 0x240248,
    0x240249, 0x241000, 0x241001, 0x241008, 0x241009, 0x241040, 0x241041, 0x241048, 0x241049,
    0x241200, 0x241201, 0x241208, 0x241209, 0x241240, 0x241241, 0x241248, 0x241249, 0x248000,
    0x248001, 0x248008, 0x248009, 0x248040, 0x248041, 0x248048, 0x248049, 0x248200, 0x248201,
    0x248208, 0x248209, 0x248240, 0x248241, 0x248248, 0x248249, 0x249000, 0x249001, 0x249008,
    0x249009, 0x249040, 0x249041, 0x249048, 0x249049, 0x249200, 0x249201, 0x249208, 0x249209,
    0x249240, 0x249241, 0x249248, 0x249249,
];

/// Encodes 10-bit voxel coordinates into 30-bit Morton code using LUT.
/// Each coordinate can be 0-1023 (10 bits).
/// The bits are interleaved: X at positions 0,3,6,...; Y at positions 1,4,7,...; Z at positions 2,5,8,...
pub fn encode_morton_code(voxel_x: u32, voxel_y: u32, voxel_z: u32) -> u32 {
    // Split each 10-bit coordinate into two 8-bit lookups (lower 8 bits + upper 2 bits)
    let x_low = (voxel_x & 0xFF) as usize;
    let x_high = ((voxel_x >> 8) & 0x03) as usize;
    let y_low = (voxel_y & 0xFF) as usize;
    let y_high = ((voxel_y >> 8) & 0x03) as usize;
    let z_low = (voxel_z & 0xFF) as usize;
    let z_high = ((voxel_z >> 8) & 0x03) as usize;

    (MORTON_LUT[x_low] | (MORTON_LUT[x_high] << 24))
        | ((MORTON_LUT[y_low] | (MORTON_LUT[y_high] << 24)) << 1)
        | ((MORTON_LUT[z_low] | (MORTON_LUT[z_high] << 24)) << 2)
}

/// Performs radix sort of a primitive set using 10-bit Morton codes.
pub struct BvhRadixSorter {
    encoded_links: Vec<EncodedLink>,
}

impl BvhRadixSorter {
    pub fn new() -> Self {
        BvhRadixSorter {
            encoded_links: Vec::new(),
        }
    }

    /// Returns the encoded links (Morton codes paired with IDs).
    pub fn encoded_links(&self) -> &[EncodedLink] {
        &self.encoded_links
    }

    /// Sorts primitives by Morton codes using radix sort (MSD - Most Significant Digit).
    pub fn perform(&mut self, links: &mut [EncodedLink]) {
        if links.is_empty() {
            return;
        }

        self.encoded_links = links.to_vec();
        Self::radix_sort_msd_static(&mut self.encoded_links, 29);
        links.copy_from_slice(&self.encoded_links);
    }

    /// Performs MSD (most significant digit) radix sort on the given range.
    fn radix_sort_msd_static(links: &mut [EncodedLink], digit: i32) {
        let start = 0;
        let end = links.len();

        Self::radix_sort_msd_impl(links, start, end, digit);
    }

    /// Recursive implementation of MSD radix sort.
    fn radix_sort_msd_impl(links: &mut [EncodedLink], start: usize, end: usize, digit: i32) {
        if start >= end || digit < 0 {
            return;
        }

        let partition_idx = Self::partition_static(links, start, end, digit);
        Self::radix_sort_msd_impl(links, start, partition_idx, digit - 1);
        Self::radix_sort_msd_impl(links, partition_idx, end, digit - 1);
    }

    /// Partitions the array by the given bit position.
    /// Returns the index where 0-bits end and 1-bits begin.
    fn partition_static(links: &mut [EncodedLink], start: usize, end: usize, digit: i32) -> usize {
        if start >= end {
            return start;
        }

        let bit_mask = 1u32 << digit;
        let mut left = start;
        let mut right = if end > 0 { end - 1 } else { 0 };

        loop {
            // Find first element with 1-bit from left
            while left < end && (links[left].code & bit_mask) == 0 {
                left += 1;
            }
            // Find first element with 0-bit from right
            while right >= start && (links[right].code & bit_mask) != 0 {
                if right == 0 {
                    break;
                }
                right -= 1;
            }

            if left >= right + 1 {
                break;
            }

            links.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }

        left
    }
}

impl Default for BvhRadixSorter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_morton_code_origin() {
        let code = encode_morton_code(0, 0, 0);
        assert_eq!(code, 0);
    }

    #[test]
    fn test_encode_morton_code_single_bits() {
        // X=1, Y=0, Z=0 should give 1 (bit 0)
        let code_x = encode_morton_code(1, 0, 0);
        assert_eq!(code_x, 1);

        // X=0, Y=1, Z=0 should give 2 (bit 1)
        let code_y = encode_morton_code(0, 1, 0);
        assert_eq!(code_y, 2);

        // X=0, Y=0, Z=1 should give 4 (bit 2)
        let code_z = encode_morton_code(0, 0, 1);
        assert_eq!(code_z, 4);

        // X=1, Y=1, Z=1 should give 7 (bits 0, 1, 2)
        let code_xyz = encode_morton_code(1, 1, 1);
        assert_eq!(code_xyz, 7);
    }

    #[test]
    fn test_encode_morton_code_interleaving() {
        // X=2, Y=0, Z=0 should give 8
        let code1 = encode_morton_code(2, 0, 0);
        assert_eq!(code1, 8);

        // X=0, Y=2, Z=0 should give 16
        let code2 = encode_morton_code(0, 2, 0);
        assert_eq!(code2, 16);

        // X=0, Y=0, Z=2 should give 32
        let code3 = encode_morton_code(0, 0, 2);
        assert_eq!(code3, 32);
    }

    #[test]
    fn test_encode_morton_code_max_values() {
        // Test with maximum 10-bit values (1023)
        let code = encode_morton_code(1023, 1023, 1023);
        // All 30 bits should be set
        assert_eq!(code, 0x3FFFFFFF);
    }

    #[test]
    fn test_encode_morton_code_symmetry() {
        let code1 = encode_morton_code(100, 200, 300);
        let code2 = encode_morton_code(200, 100, 300);
        let code3 = encode_morton_code(100, 300, 200);

        // Codes should be different
        assert_ne!(code1, code2);
        assert_ne!(code1, code3);
        assert_ne!(code2, code3);
    }

    #[test]
    fn test_encode_morton_code_ordering() {
        let code_000 = encode_morton_code(0, 0, 0);
        let code_100 = encode_morton_code(1, 0, 0);
        let code_001023 = encode_morton_code(0, 0, 1023);

        let diff1 = code_100 - code_000;
        let diff2 = code_001023 - code_000;

        assert!(diff1 < diff2);
    }

    #[test]
    fn test_encode_morton_code_mid_values() {
        let code = encode_morton_code(512, 512, 512);
        // Should be non-zero and within 30-bit range
        assert!(code > 0);
        assert!(code <= 0x3FFFFFFF);
    }

    #[test]
    fn test_encode_morton_code_powers_of_two() {
        let code4 = encode_morton_code(4, 0, 0);
        let code8 = encode_morton_code(8, 0, 0);
        let code16 = encode_morton_code(16, 0, 0);

        // Each should be 8x the previous (3 bit positions apart)
        assert_eq!(code8, code4 * 8);
        assert_eq!(code16, code8 * 8);
    }

    #[test]
    fn test_encode_morton_code_consistent_encoding() {
        for _ in 0..100 {
            let code1 = encode_morton_code(100, 200, 300);
            let code2 = encode_morton_code(100, 200, 300);
            assert_eq!(code1, code2);
        }
    }

    #[test]
    fn test_encode_morton_code_boundary_values() {
        let code1022 = encode_morton_code(1022, 1022, 1022);
        let code1023 = encode_morton_code(1023, 1023, 1023);

        assert!(code1022 < code1023);
        assert_eq!(code1023, 0x3FFFFFFF);
    }

    #[test]
    fn test_encode_morton_code_small_values() {
        let code2 = encode_morton_code(2, 2, 2);
        let code3 = encode_morton_code(3, 3, 3);

        assert!(code2 < 100);
        assert!(code3 < 100);
        assert!(code2 < code3);
    }

    #[test]
    fn test_encode_morton_code_single_axis_values() {
        let code_x = encode_morton_code(255, 0, 0);
        let code_y = encode_morton_code(0, 255, 0);
        let code_z = encode_morton_code(0, 0, 255);

        assert_ne!(code_x, code_y);
        assert_ne!(code_y, code_z);
        assert_ne!(code_x, code_z);
    }

    #[test]
    fn test_encode_morton_code_sequential_x_values() {
        let mut prev = encode_morton_code(0, 500, 500);
        for x in 1..=10 {
            let curr = encode_morton_code(x, 500, 500);
            assert!(curr > prev);
            prev = curr;
        }
    }

    #[test]
    fn test_encode_morton_code_sequential_y_values() {
        let mut prev = encode_morton_code(500, 0, 500);
        for y in 1..=10 {
            let curr = encode_morton_code(500, y, 500);
            assert!(curr > prev);
            prev = curr;
        }
    }

    #[test]
    fn test_encode_morton_code_sequential_z_values() {
        let mut prev = encode_morton_code(500, 500, 0);
        for z in 1..=10 {
            let curr = encode_morton_code(500, 500, z);
            assert!(curr > prev);
            prev = curr;
        }
    }

    #[test]
    fn test_encode_morton_code_xy_equal() {
        let code1 = encode_morton_code(1, 1, 0);
        let code2 = encode_morton_code(2, 2, 0);
        let code3 = encode_morton_code(4, 4, 0);

        assert_eq!(code1, 3);
        assert_eq!(code2, 24);
        assert_eq!(code3, 192);
    }

    #[test]
    fn test_encode_morton_code_all_axes_equal() {
        let code1 = encode_morton_code(1, 1, 1);
        let code2 = encode_morton_code(2, 2, 2);

        assert_eq!(code1, 7);
        assert_eq!(code2, 56);
    }

    #[test]
    fn test_encode_morton_code_half_max() {
        let code = encode_morton_code(512, 512, 512);
        assert!(code > 0x1FFFFFFF);
        assert!(code <= 0x3FFFFFFF);
    }

    #[test]
    fn test_encode_morton_code_quarter_max() {
        let code = encode_morton_code(256, 256, 256);
        assert!(code > 0);
        assert!(code < 0x3FFFFFFF);
    }

    #[test]
    fn test_encode_morton_code_different_scales() {
        let code1 = encode_morton_code(100, 100, 100);
        let code2 = encode_morton_code(200, 200, 200);
        assert!(code2 > code1);
    }

    #[test]
    fn test_encode_morton_code_adjacent_cells_x() {
        let code1 = encode_morton_code(100, 100, 100);
        let code2 = encode_morton_code(101, 100, 100);
        assert_eq!(code2 - code1, 1);
    }

    #[test]
    fn test_encode_morton_code_adjacent_cells_y() {
        let code1 = encode_morton_code(100, 100, 100);
        let code2 = encode_morton_code(100, 101, 100);
        assert_eq!(code2 - code1, 2);
    }

    #[test]
    fn test_encode_morton_code_adjacent_cells_z() {
        let code1 = encode_morton_code(100, 100, 100);
        let code2 = encode_morton_code(100, 100, 101);
        assert_eq!(code2 - code1, 4);
    }

    #[test]
    fn test_encode_morton_code_monotonic_x() {
        let mut prev = 0;
        for x in 0..100 {
            let code = encode_morton_code(x, 0, 0);
            assert!(code >= prev);
            prev = code;
        }
    }

    #[test]
    fn test_encode_morton_code_monotonic_y() {
        let mut prev = 0;
        for y in 0..100 {
            let code = encode_morton_code(0, y, 0);
            assert!(code >= prev);
            prev = code;
        }
    }

    #[test]
    fn test_encode_morton_code_monotonic_z() {
        let mut prev = 0;
        for z in 0..100 {
            let code = encode_morton_code(0, 0, z);
            assert!(code >= prev);
            prev = code;
        }
    }

    #[test]
    fn test_encode_morton_code_no_overflow() {
        let code = encode_morton_code(1023, 1023, 1023);
        assert_eq!(code & 0xC0000000, 0);
    }

    #[test]
    fn test_encode_morton_code_unique_for_different_inputs() {
        let code1 = encode_morton_code(100, 200, 300);
        let code2 = encode_morton_code(100, 200, 301);
        let code3 = encode_morton_code(100, 201, 300);
        let code4 = encode_morton_code(101, 200, 300);

        assert_ne!(code1, code2);
        assert_ne!(code1, code3);
        assert_ne!(code1, code4);
        assert_ne!(code2, code3);
        assert_ne!(code2, code4);
        assert_ne!(code3, code4);
    }

    #[test]
    fn test_radix_sorter_basic() {
        let mut links = vec![
            EncodedLink::new(100, 0),
            EncodedLink::new(50, 1),
            EncodedLink::new(75, 2),
        ];

        let mut sorter = BvhRadixSorter::new();
        sorter.perform(&mut links);

        // Should be sorted by code
        assert_eq!(links[0].code, 50);
        assert_eq!(links[1].code, 75);
        assert_eq!(links[2].code, 100);

        // IDs should track their original positions
        assert_eq!(links[0].id, 1);
        assert_eq!(links[1].id, 2);
        assert_eq!(links[2].id, 0);
    }

    #[test]
    fn test_radix_sorter_empty() {
        let mut links: Vec<EncodedLink> = vec![];
        let mut sorter = BvhRadixSorter::new();
        sorter.perform(&mut links); // Should not crash
    }

    #[test]
    fn test_radix_sorter_single_element() {
        let mut links = vec![EncodedLink::new(42, 0)];
        let mut sorter = BvhRadixSorter::new();
        sorter.perform(&mut links);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].code, 42);
        assert_eq!(links[0].id, 0);
    }

    #[test]
    fn test_radix_sorter_already_sorted() {
        let mut links = (0..10)
            .map(|i| EncodedLink::new(i as u32 * 10, i))
            .collect::<Vec<_>>();

        let mut sorter = BvhRadixSorter::new();
        sorter.perform(&mut links);

        // Should remain sorted
        for i in 0..9 {
            assert!(links[i].code <= links[i + 1].code);
        }
    }

    #[test]
    fn test_radix_sorter_reverse() {
        let mut links = (0..10)
            .rev()
            .map(|i| EncodedLink::new(i as u32 * 10, i))
            .collect::<Vec<_>>();

        let mut sorter = BvhRadixSorter::new();
        sorter.perform(&mut links);

        // Should be sorted
        for i in 0..9 {
            assert!(links[i].code <= links[i + 1].code);
        }
    }

    #[test]
    fn test_radix_sorter_duplicate_codes() {
        let mut links = vec![
            EncodedLink::new(100, 0),
            EncodedLink::new(100, 1),
            EncodedLink::new(50, 2),
            EncodedLink::new(100, 3),
        ];

        let mut sorter = BvhRadixSorter::new();
        sorter.perform(&mut links);

        // Should be sorted, duplicates maintain relative order
        assert_eq!(links[0].code, 50);
        for i in 1..4 {
            assert_eq!(links[i].code, 100);
        }
    }
}
