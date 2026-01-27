impl Solution {
    /// Finds minimum distances between node pairs in an implicit graph.
    ///
    /// # Intuition
    /// Nodes are connected if their values differ by at most maxDiff. When sorted by value,
    /// connected components form contiguous segments. The minimum distance between two nodes
    /// in the same component equals the minimum number of hops needed, which can be found
    /// using binary lifting on the sorted positions.
    ///
    /// # Approach
    /// 1. Sort nodes by their values while tracking original indices
    /// 2. For each position, find the rightmost reachable position (farthest jump)
    /// 3. Build binary lifting table: jump[k][i] = position after 2^k jumps from i
    /// 4. For each query, use binary lifting to find minimum hops between nodes
    ///
    /// # Complexity
    /// - Time: O(n log n + q log n) for sorting, preprocessing, and queries
    /// - Space: O(n log n) for the binary lifting table
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;

        // Sort indices by their corresponding values
        let mut sorted_indices: Vec<usize> = (0..n).collect();
        sorted_indices.sort_unstable_by_key(|&i| nums[i]);

        // Map original index to position in sorted order
        let mut pos = vec![0usize; n];
        for (p, &idx) in sorted_indices.iter().enumerate() {
            pos[idx] = p;
        }

        // For each position, find the farthest position reachable in one hop
        // Using two pointers since sorted values allow binary search
        let mut farthest = vec![0usize; n];
        let mut right = 0usize;
        for left in 0..n {
            while right < n && nums[sorted_indices[right]] - nums[sorted_indices[left]] <= max_diff
            {
                right += 1;
            }
            farthest[left] = right - 1;
        }

        // Binary lifting: jump[k][i] = position after 2^k maximum jumps from position i
        let log_n = (n.max(1) as f64).log2().ceil() as usize + 1;
        let mut jump = vec![vec![0usize; n]; log_n];

        // Base case: jump[0][i] = farthest reachable in one hop
        jump[0].copy_from_slice(&farthest);

        // Build sparse table
        for k in 1..log_n {
            for i in 0..n {
                jump[k][i] = jump[k - 1][jump[k - 1][i]];
            }
        }

        // Process queries
        queries
            .iter()
            .map(|q| {
                let (u, v) = (q[0] as usize, q[1] as usize);
                if u == v {
                    return 0;
                }

                // Get positions in sorted order, ensure left < right
                let (mut left, right) = if pos[u] < pos[v] {
                    (pos[u], pos[v])
                } else {
                    (pos[v], pos[u])
                };

                // Check if already reachable in one hop
                if farthest[left] >= right {
                    return 1;
                }

                // Binary lifting to find minimum hops
                let mut hops = 0i32;
                for k in (0..log_n).rev() {
                    if jump[k][left] < right {
                        left = jump[k][left];
                        hops += 1 << k;
                    }
                }

                // Final hop to reach right
                if farthest[left] >= right {
                    hops + 1
                } else {
                    -1 // Not reachable
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::path_existence_queries(
                5,
                vec![1, 8, 3, 4, 2],
                3,
                vec![vec![0, 3], vec![2, 4]]
            ),
            vec![1, 1]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::path_existence_queries(
                5,
                vec![5, 3, 1, 9, 10],
                2,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![4, 3]]
            ),
            vec![1, 2, -1, 1]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::path_existence_queries(
                3,
                vec![3, 6, 1],
                1,
                vec![vec![0, 0], vec![0, 1], vec![1, 2]]
            ),
            vec![0, -1, -1]
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(
            Solution::path_existence_queries(1, vec![5], 10, vec![vec![0, 0]]),
            vec![0]
        );
    }

    #[test]
    fn test_all_connected() {
        assert_eq!(
            Solution::path_existence_queries(4, vec![1, 2, 3, 4], 1, vec![vec![0, 3], vec![1, 2]]),
            vec![3, 1]
        );
    }
}
