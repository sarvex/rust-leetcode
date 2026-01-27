use std::cmp::Ordering;

impl Solution {
    /// Sorted array with two-pointer scan for finding all unique triplets summing to zero.
    ///
    /// # Intuition
    /// Sorting enables skipping duplicates and using a two-pointer technique
    /// on the remaining subarray for each fixed first element, reducing the
    /// problem from O(n^3) to O(n^2).
    ///
    /// # Approach
    /// Sort the array. For each element `nums[i]` (while non-positive to allow
    /// a valid triplet), use two pointers `left` and `right` converging from
    /// both ends of the remaining subarray. Skip duplicate values at each level
    /// to ensure unique triplets.
    ///
    /// # Complexity
    /// - Time: O(n^2) — outer loop × two-pointer inner scan
    /// - Space: O(log n) — sorting stack (excluding output)
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut result = Vec::new();
        let mut i = 0;

        while i < n.saturating_sub(2) && nums[i] <= 0 {
            let (mut left, mut right) = (i + 1, n - 1);

            while left < right {
                match (nums[i] + nums[left] + nums[right]).cmp(&0) {
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < n && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while right > 0 && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                }
            }

            i += 1;
            while i < n.saturating_sub(2) && nums[i] == nums[i - 1] {
                i += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        result.sort();
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn no_triplets() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn all_zeros() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn too_few_elements() {
        assert_eq!(Solution::three_sum(vec![1, 2]), Vec::<Vec<i32>>::new());
    }
}
