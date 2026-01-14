impl Solution {
    /// Count Increasing Quadruplets
    ///
    /// # Intuition
    /// A quadruplet (i, j, k, l) is valid when nums[i] < nums[k] < nums[j] < nums[l].
    /// Notice the "132-like" pattern where the middle elements are swapped.
    /// For each valid (j, k) pair where j < k but nums[k] < nums[j], we count
    /// valid i and l positions independently and multiply.
    ///
    /// # Approach
    /// - For each j, iterate k from n-1 down to j+1
    /// - Track count of elements > nums[j] to the right of k (for valid l positions)
    /// - Use a Binary Indexed Tree to efficiently count elements < nums[k] to the left of j
    /// - For each valid (j, k) pair, add product of left and right counts to result
    ///
    /// # Complexity
    /// - Time: O(n² log n) - nested loops with BIT operations
    /// - Space: O(n) - BIT storage
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut result: i64 = 0;
        let mut bit = BinaryIndexedTree::new(n + 2);

        for j in 0..n.saturating_sub(2) {
            let mut greater_count: i64 = 0;

            for k in (j + 1..n).rev() {
                if nums[k] < nums[j] {
                    let left_count = bit.query(nums[k] as usize);
                    result += left_count * greater_count;
                }

                if nums[k] > nums[j] {
                    greater_count += 1;
                }
            }

            bit.update(nums[j] as usize);
        }

        result
    }
}

struct BinaryIndexedTree {
    tree: Vec<i64>,
}

impl BinaryIndexedTree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size],
        }
    }

    fn update(&mut self, mut index: usize) {
        while index < self.tree.len() {
            self.tree[index] += 1;
            index += index & index.wrapping_neg();
        }
    }

    fn query(&self, mut index: usize) -> i64 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & index.wrapping_neg();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 2, 4, 5];
        assert_eq!(Solution::count_quadruplets(nums), 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::count_quadruplets(nums), 0);
    }

    #[test]
    fn test_minimum_length() {
        let nums = vec![2, 4, 1, 3];
        assert_eq!(Solution::count_quadruplets(nums), 0);
    }

    #[test]
    fn test_two_quadruplets() {
        // (0,1,2,4): 1 < 2 < 4 < 5 ✓
        // (0,1,3,4): 1 < 3 < 4 < 5 ✓
        let nums = vec![1, 4, 2, 3, 5];
        assert_eq!(Solution::count_quadruplets(nums), 2);
    }

    #[test]
    fn test_reverse_sorted() {
        let nums = vec![4, 3, 2, 1];
        assert_eq!(Solution::count_quadruplets(nums), 0);
    }

    #[test]
    fn test_longer_sequence() {
        // (0,1,2,5): 1 < 2 < 5 < 6 ✓
        // (0,1,3,5): 1 < 4 < 5 < 6 ✓
        // (0,1,4,5): 1 < 3 < 5 < 6 ✓
        // (0,3,4,5): 1 < 3 < 4 < 6 ✓
        // (2,3,4,5): 2 < 3 < 4 < 6 ✓
        let nums = vec![1, 5, 2, 4, 3, 6];
        assert_eq!(Solution::count_quadruplets(nums), 5);
    }
}
