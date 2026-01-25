/// Subtree Inversion Sum - Optimized Bottom-Up Tree DP
///
/// # Intuition
/// Track two states per node: `f[d]` for normal parity and `g[d]` for inverted parity,
/// where `d` is the distance from the nearest inverted ancestor. When `d >= k`, we can
/// choose to invert; otherwise we cannot.
///
/// # Approach
/// Use bottom-up tree DP with iterative post-order traversal:
/// - `f[node][d]` = max sum of subtree with parity=0 (normal), distance=d
/// - `g[node][d]` = max sum of subtree with parity=1 (inverted), distance=d
///
/// For distance d < k (cannot invert):
/// - `f[node][d] = nums[node] + sum(f[child][d+1])`
/// - `g[node][d] = -nums[node] + sum(g[child][d+1])`
///
/// For distance d = k (can invert):
/// - `f[node][k] = max(nums[node] + sum(f[child][k]), -nums[node] + sum(g[child][1]))`
/// - `g[node][k] = max(-nums[node] + sum(g[child][k]), nums[node] + sum(f[child][1]))`
///
/// # Complexity
/// - Time: O(n * k)
/// - Space: O(n * k)

impl Solution {
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;

        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut f = vec![vec![0i64; k + 1]; n];
        let mut g = vec![vec![0i64; k + 1]; n];

        // Iterative post-order traversal
        let mut stack = vec![(0usize, n, false)];
        let mut order = Vec::with_capacity(n);

        while let Some((node, parent, visited)) = stack.pop() {
            if visited {
                order.push((node, parent));
            } else {
                stack.push((node, parent, true));
                for &child in &adj[node] {
                    if child != parent {
                        stack.push((child, node, false));
                    }
                }
            }
        }

        // Process nodes in post-order (children before parents)
        for (node, parent) in order {
            let val = nums[node] as i64;

            // Compute sums from children for all distance values
            let mut sf = vec![0i64; k + 1];
            let mut sg = vec![0i64; k + 1];

            for &child in &adj[node] {
                if child != parent {
                    for d in 1..=k {
                        sf[d] += f[child][d];
                        sg[d] += g[child][d];
                    }
                }
            }

            // Distance k: can choose to invert or not
            f[node][k] = (val + sf[k]).max(-val + sg[1]);
            g[node][k] = (-val + sg[k]).max(val + sf[1]);

            // Distance 1 to k-1: cannot invert
            for d in (1..k).rev() {
                f[node][d] = val + sf[d + 1];
                g[node][d] = -val + sg[d + 1];
            }
        }

        f[0][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ];
        let nums = vec![4, -8, -6, 3, 7, -2, 5];
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, 2), 27);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let nums = vec![-1, 3, -2, 4, -5];
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, 2), 9);
    }

    #[test]
    fn test_example_3() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let nums = vec![0, -1, -2];
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, 3), 3);
    }

    #[test]
    fn test_single_edge() {
        let edges = vec![vec![0, 1]];
        let nums = vec![-5, -10];
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, 1), 15);
    }

    #[test]
    fn test_all_positive() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, 1), 6);
    }

    #[test]
    fn test_k_equals_one() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let nums = vec![-1, -2, -3];
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, 1), 6);
    }

    #[test]
    fn test_large_k() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let nums = vec![-1, -2, -3, -4];
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, 50), 10);
    }
}
