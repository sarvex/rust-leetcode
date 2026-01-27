impl Solution {
    /// Maximize marked indices by greedy two-pointer pairing.
    ///
    /// # Intuition
    /// After sorting, pair the smallest element with a candidate from the upper half.
    /// If 2 * nums[i] <= nums[j], the pair is valid. This greedy strategy maximizes pairs.
    ///
    /// # Approach
    /// 1. Sort the array
    /// 2. Use pointer i starting at 0, iterate j from the midpoint
    /// 3. Advance i when a valid pair is found
    /// 4. Return 2 * i (each pair marks two indices)
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) auxiliary
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut i = 0;

        for j in (n + 1) / 2..n {
            if nums[i] * 2 <= nums[j] {
                i += 1;
            }
        }

        (i * 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]), 2);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::max_num_of_marked_indices(vec![1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_perfect_pairs() {
        assert_eq!(Solution::max_num_of_marked_indices(vec![1, 2, 3, 6]), 4);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(
            Solution::max_num_of_marked_indices(vec![
                1, 78, 27, 48, 14, 86, 79, 68, 77, 20, 57, 21, 18, 67, 5, 51, 70, 85, 47, 56, 22,
                79, 41, 8, 39, 81, 59
            ]),
            26
        );
    }
}
