impl Solution {
    /// Counts rods that have all three colored rings using bitmask tracking.
    ///
    /// # Intuition
    /// Each rod needs exactly one red, one green, and one blue ring. By
    /// assigning each color a unique bit (R=1, G=2, B=4), a rod with all
    /// three colors will have bitmask 7 (0b111).
    ///
    /// # Approach
    /// 1. Parse the input string in pairs: color character followed by rod digit.
    /// 2. OR the corresponding color bit into the rod's bitmask.
    /// 3. Count rods whose bitmask equals 7.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of the rings string
    /// - Space: O(1) — fixed 10-element array
    pub fn count_points(rings: String) -> i32 {
        let bytes = rings.as_bytes();
        let mut mask = [0u8; 10];

        bytes.chunks_exact(2).for_each(|pair| {
            let bit = match pair[0] {
                b'R' => 1,
                b'G' => 2,
                b'B' => 4,
                _ => 0,
            };
            mask[(pair[1] - b'0') as usize] |= bit;
        });

        mask.iter().filter(|m| **m == 7).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_colors_on_one_rod() {
        assert_eq!(Solution::count_points("B0R0G0".to_string()), 1);
    }

    #[test]
    fn test_multiple_rods() {
        assert_eq!(Solution::count_points("B0B6G0R6R0R6G6".to_string()), 1);
    }

    #[test]
    fn test_no_complete_rod() {
        assert_eq!(Solution::count_points("B0R1G2".to_string()), 0);
    }
}
