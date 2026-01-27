impl Solution {
    /// Count increasing quadruplets using BIT-accelerated enumeration.
    ///
    /// # Intuition
    /// A quadruplet (i, j, k, l) is valid when nums[i] < nums[k] < nums[j] < nums[l].
    /// This "132-like" pattern means for each (j, k) pair where j < k but nums[k] < nums[j],
    /// we independently count valid i and l positions and multiply.
    ///
    /// # Approach
    /// 1. For each j, iterate k from n-1 down to j+1
    /// 2. Track count of elements > nums[j] to the right of k (valid l positions)
    /// 3. Use a Binary Indexed Tree to count elements < nums[k] to the left of j
    /// 4. For each valid (j, k) pair, add product of left and right counts
    ///
    /// # Complexity
    /// - Time: O(n² log n)
    /// - Space: O(n)
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut result: i64 = 0;
        let mut bit = BinaryIndexedTree::new(n + 2);

        for j in 0..n.saturating_sub(2) {
            let mut greater_count: i64 = 0;

            for k in (j + 1..n).rev() {
                if nums[k] < nums[j] {
                    result += bit.query(nums[k] as usize) * greater_count;
                } else if nums[k] > nums[j] {
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
    fn test_basic_quadruplet() {
        assert_eq!(Solution::count_quadruplets(vec![1, 3, 2, 4, 5]), 2);
    }

    #[test]
    fn test_strictly_increasing_no_quadruplet() {
        assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::count_quadruplets(vec![2, 4, 1, 3]), 0);
    }

    #[test]
    fn test_reverse_sorted() {
        assert_eq!(Solution::count_quadruplets(vec![4, 3, 2, 1]), 0);
    }

    #[test]
    fn test_longer_sequence() {
        assert_eq!(Solution::count_quadruplets(vec![1, 5, 2, 4, 3, 6]), 5);
    }
}
