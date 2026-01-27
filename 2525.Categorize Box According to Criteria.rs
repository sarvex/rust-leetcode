impl Solution {
    /// Categorizes a box as "Bulky", "Heavy", "Both", or "Neither".
    ///
    /// # Intuition
    /// Bulky if any dimension >= 10^4 or volume >= 10^9. Heavy if mass >= 100.
    /// Use a bitmask to combine both conditions.
    ///
    /// # Approach
    /// Compute volume with i64 to avoid overflow. Set bit 0 for bulky, bit 1 for heavy.
    /// Index into a category array.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let volume = length as i64 * width as i64 * height as i64;

        let bulky =
            length >= 10_000 || width >= 10_000 || height >= 10_000 || volume >= 1_000_000_000;
        let heavy = mass >= 100;

        match (bulky, heavy) {
            (true, true) => "Both",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
            (false, false) => "Neither",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both() {
        assert_eq!(Solution::categorize_box(10000, 10000, 10000, 100), "Both");
    }

    #[test]
    fn test_bulky_only() {
        assert_eq!(Solution::categorize_box(10000, 1, 1, 1), "Bulky");
    }

    #[test]
    fn test_heavy_only() {
        assert_eq!(Solution::categorize_box(1, 1, 1, 100), "Heavy");
    }

    #[test]
    fn test_neither() {
        assert_eq!(Solution::categorize_box(1, 1, 1, 1), "Neither");
    }
}
