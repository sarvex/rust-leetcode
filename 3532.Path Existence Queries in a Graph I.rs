impl Solution {
    /// Group-based connectivity using sorted array structure.
    ///
    /// # Intuition
    /// Since `nums` is sorted, an edge exists between nodes `i` and `j` iff
    /// `nums[j] - nums[i] <= maxDiff`. Connected components are therefore
    /// contiguous index ranges — whenever two adjacent nodes `i` and `i+1`
    /// have `nums[i+1] - nums[i] > maxDiff`, a new component begins.
    /// We assign each node a monotonically increasing group ID and answer
    /// every query in O(1) by comparing group IDs.
    ///
    /// # Approach
    /// 1. Build a `group` array: start all nodes in group 0, increment the
    ///    group counter each time adjacent nodes exceed `maxDiff`.
    /// 2. For each query `[u, v]`, nodes are connected iff `group[u] == group[v]`.
    ///
    /// # Complexity
    /// - Time: O(n + q) where q = queries.length
    /// - Space: O(n) for the group array
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut group = vec![0u16; n];
        let mut id = 0u16;

        // Build groups with raw loop for maximum speed
        for i in 1..n {
            if unsafe { *nums.get_unchecked(i) - *nums.get_unchecked(i - 1) } > max_diff {
                id += 1;
            }
            unsafe { *group.get_unchecked_mut(i) = id };
        }

        // Process queries with preallocated result
        let mut result = Vec::with_capacity(queries.len());
        for q in queries {
            let u = unsafe { *q.get_unchecked(0) } as usize;
            let v = unsafe { *q.get_unchecked(1) } as usize;
            result.push(unsafe { *group.get_unchecked(u) == *group.get_unchecked(v) });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::path_existence_queries(2, vec![1, 3], 1, vec![vec![0, 0], vec![0, 1]]),
            vec![true, false]
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::path_existence_queries(
                4,
                vec![2, 5, 6, 8],
                2,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]
            ),
            vec![false, false, true, true]
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(
            Solution::path_existence_queries(1, vec![42], 0, vec![vec![0, 0]]),
            vec![true]
        );
    }

    #[test]
    fn test_all_connected() {
        // Every adjacent pair within maxDiff — all one component
        assert_eq!(
            Solution::path_existence_queries(4, vec![1, 2, 3, 4], 1, vec![vec![0, 3], vec![1, 3]]),
            vec![true, true]
        );
    }

    #[test]
    fn test_all_disconnected() {
        // Gaps > maxDiff between every adjacent pair
        assert_eq!(
            Solution::path_existence_queries(
                3,
                vec![0, 10, 20],
                5,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]]
            ),
            vec![false, false, false]
        );
    }

    #[test]
    fn test_max_diff_zero() {
        // Only nodes with identical values are connected
        assert_eq!(
            Solution::path_existence_queries(
                4,
                vec![1, 1, 2, 2],
                0,
                vec![vec![0, 1], vec![2, 3], vec![0, 2]]
            ),
            vec![true, true, false]
        );
    }
}
