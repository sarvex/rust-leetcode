impl Solution {
    /// Counts unique XOR values from all triplets (i, j, k) where i ≤ j ≤ k.
    ///
    /// # Intuition
    /// Since nums[i] ≤ 1500, all XOR values fit within the next power of two above
    /// the maximum value (at most 2048). A flat boolean array indexed by XOR value is
    /// far cheaper than a HashSet — O(1) lookup and insertion with no hashing overhead
    /// and excellent cache locality.
    ///
    /// # Approach
    /// 1. Find `u` = smallest power of two > max(nums), bounding all XOR values.
    /// 2. Enumerate all pairs (i ≤ j) and mark `pairs[nums[i] ^ nums[j]] = true`.
    /// 3. For each reachable pair XOR value `x`, mark `triplets[x ^ v] = true`
    ///    for every `v` in nums.
    /// 4. Count true entries in `triplets`.
    ///
    /// The pair step is O(n²) but n ≤ 1500, so at most 1.125 M iterations.
    /// The triplet step iterates over at most `u ≤ 2048` pair values × n ≤ 1500
    /// elements = ≤ 3 M iterations, each being a single array write.
    ///
    /// # Complexity
    /// - Time: O(n² + u·n) where u ≤ 2048
    /// - Space: O(u) = O(1) — two boolean arrays of at most 2048 entries
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max_val = nums.iter().max().copied().unwrap_or(0) as usize;

        // Next power of two strictly above max_val bounds all possible XOR values.
        let mut u = 1usize;
        while u <= max_val {
            u <<= 1;
        }

        // Mark all pairwise XOR values (i ≤ j covers all unordered pairs).
        let mut pairs = vec![false; u];
        for i in 0..n {
            for j in i..n {
                pairs[(nums[i] ^ nums[j]) as usize] = true;
            }
        }

        // For each reachable pair XOR, extend to triplet XORs by XORing with each element.
        let mut triplets = vec![false; u];
        for (x, &reached) in pairs.iter().enumerate() {
            if reached {
                for &v in &nums {
                    triplets[x ^ v as usize] = true;
                }
            }
        }

        triplets.iter().filter(|&&b| b).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // (0,0,0)->1, (0,0,1)->3, (0,1,1)->1, (1,1,1)->3 => {1,3}
        assert_eq!(Solution::unique_xor_triplets(vec![1, 3]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::unique_xor_triplets(vec![6, 7, 8, 9]), 4);
    }

    #[test]
    fn test_single_element() {
        // Only triplet is (0,0,0): 5^5^5 = 5
        assert_eq!(Solution::unique_xor_triplets(vec![5]), 1);
    }

    #[test]
    fn test_two_identical_elements() {
        // pair_xors = {0}, 0^2 = 2 => triplets = {2}
        assert_eq!(Solution::unique_xor_triplets(vec![2, 2]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::unique_xor_triplets(vec![4, 4, 4, 4]), 1);
    }

    #[test]
    fn test_power_of_two_elements() {
        // nums = [1, 2, 4]: unique triplet XORs = {1, 2, 4, 7}
        assert_eq!(Solution::unique_xor_triplets(vec![1, 2, 4]), 4);
    }

    #[test]
    fn test_large_uniform() {
        // All same value — only one unique XOR value
        assert_eq!(Solution::unique_xor_triplets(vec![1500; 1500]), 1);
    }

    #[test]
    fn test_stress_brute_force() {
        // Validate against exhaustive brute force for a small input
        let nums = vec![3, 5, 6, 10, 15];
        let n = nums.len();
        let mut expected = std::collections::HashSet::new();
        for i in 0..n {
            for j in i..n {
                for k in j..n {
                    expected.insert(nums[i] ^ nums[j] ^ nums[k]);
                }
            }
        }
        assert_eq!(Solution::unique_xor_triplets(nums), expected.len() as i32);
    }
}
