pub struct Solution;

impl Solution {
    /// Computes the total Hamming distance between all pairs using bit counting.
    ///
    /// # Intuition
    /// For each bit position, if `k` numbers have that bit set and `n - k` do
    /// not, those bits contribute `k * (n - k)` to the total distance.
    ///
    /// # Approach
    /// 1. For each of the 32 bit positions, count how many numbers have the bit set.
    /// 2. Multiply the set count by the unset count and accumulate.
    ///
    /// # Complexity
    /// - Time: O(32 · n) = O(n)
    /// - Space: O(1)
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        (0..32).fold(0, |total, bit| {
            let ones = nums.iter().filter(|x| (*x >> bit) & 1 == 1).count() as i32;
            total + ones * (n - ones)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::total_hamming_distance(vec![4, 14, 2]), 6);
    }

    #[test]
    fn test_identical() {
        assert_eq!(Solution::total_hamming_distance(vec![3, 3, 3]), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::total_hamming_distance(vec![1, 4]), 3);
    }
}
