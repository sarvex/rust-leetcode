impl Solution {
    /// Sort by popcount with value as tiebreaker using packed key.
    ///
    /// # Intuition
    /// Encoding both popcount and value into a single integer eliminates
    /// tuple comparison overhead. Since values are at most 10,000 (14 bits),
    /// shifting popcount left by 16 bits produces a composite key where
    /// natural integer ordering matches the desired sort order.
    ///
    /// # Approach
    /// 1. Pack sort key as `(count_ones << 16) | value` into one `i32`
    /// 2. Sort with `sort_unstable_by_key` on the packed key
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) in-place sort
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by_key(|&x| (x.count_ones() << 16) | x as u32);
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_bits() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
    }

    #[test]
    fn large_values() {
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
