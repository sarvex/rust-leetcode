impl Solution {
    /// Maximum Segment Sum After Removals
    ///
    /// # Intuition
    /// Processing removals directly is inefficient. Instead, reverse the process:
    /// start with an empty array and add elements back in reverse removal order.
    /// When adding an element, merge with adjacent segments using Union-Find.
    ///
    /// # Approach
    /// 1. Use Union-Find (Disjoint Set Union) with path compression
    /// 2. Process queries in reverse order (additions instead of removals)
    /// 3. For each addition, merge with adjacent segments if they exist
    /// 4. Track maximum segment sum at each step and build answer in reverse
    ///
    /// # Complexity
    /// - Time: O(n × α(n)) where α is inverse Ackermann function (effectively O(n))
    /// - Space: O(n) for Union-Find structures and auxiliary arrays
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut segment_sum: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let mut present = vec![false; n];
        let mut answer = vec![0i64; n];
        let mut max_sum: i64 = 0;

        // Process removals in reverse order (as additions)
        for i in (1..n).rev() {
            let idx = remove_queries[i] as usize;
            present[idx] = true;

            // Merge with left neighbor if present
            if idx > 0 && present[idx - 1] {
                Self::union(&mut parent, &mut segment_sum, idx - 1, idx);
            }

            // Merge with right neighbor if present
            if idx + 1 < n && present[idx + 1] {
                Self::union(&mut parent, &mut segment_sum, idx + 1, idx);
            }

            // Update maximum with current segment sum
            let root = Self::find(&mut parent, idx);
            max_sum = max_sum.max(segment_sum[root]);
            answer[i - 1] = max_sum;
        }

        answer
    }

    /// Find root with path compression for near-constant time lookups
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }

    /// Union two segments by merging their sums into a single component
    fn union(parent: &mut [usize], segment_sum: &mut [i64], x: usize, y: usize) {
        let root_x = Self::find(parent, x);
        let root_y = Self::find(parent, y);

        if root_x != root_y {
            parent[root_x] = root_y;
            segment_sum[root_y] += segment_sum[root_x];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 5, 6, 1];
        let remove_queries = vec![0, 3, 2, 4, 1];
        let expected = vec![14, 7, 2, 2, 0];
        assert_eq!(
            Solution::maximum_segment_sum(nums, remove_queries),
            expected
        );
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 2, 11, 1];
        let remove_queries = vec![3, 2, 1, 0];
        let expected = vec![16, 5, 3, 0];
        assert_eq!(
            Solution::maximum_segment_sum(nums, remove_queries),
            expected
        );
    }

    #[test]
    fn test_single_element() {
        let nums = vec![5];
        let remove_queries = vec![0];
        let expected = vec![0];
        assert_eq!(
            Solution::maximum_segment_sum(nums, remove_queries),
            expected
        );
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![1, 2];
        let remove_queries = vec![0, 1];
        let expected = vec![2, 0];
        assert_eq!(
            Solution::maximum_segment_sum(nums, remove_queries),
            expected
        );
    }

    #[test]
    fn test_remove_middle_first() {
        let nums = vec![1, 10, 1];
        let remove_queries = vec![1, 0, 2];
        let expected = vec![1, 1, 0];
        assert_eq!(
            Solution::maximum_segment_sum(nums, remove_queries),
            expected
        );
    }

    #[test]
    fn test_large_values() {
        let nums = vec![1_000_000_000, 1_000_000_000, 1_000_000_000];
        let remove_queries = vec![1, 0, 2];
        let expected = vec![1_000_000_000, 1_000_000_000, 0];
        assert_eq!(
            Solution::maximum_segment_sum(nums, remove_queries),
            expected
        );
    }
}
