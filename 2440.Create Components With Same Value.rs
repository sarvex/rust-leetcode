impl Solution {
    /// Finds the maximum number of edges to remove to create equal-sum components.
    ///
    /// # Intuition
    /// Instead of running DFS for each potential target, compute all subtree sums
    /// in one pass. A valid partition into k components requires each subtree sum
    /// to be a multiple of the target sum S/k.
    ///
    /// # Approach
    /// 1. Build adjacency list and compute total sum S
    /// 2. Single DFS to count frequency of each subtree sum
    /// 3. Find divisors of S in O(sqrt(S))
    /// 4. For each divisor k (where k <= n), check if k components can be formed
    /// 5. Return maximum valid components - 1
    ///
    /// # Complexity
    /// - Time: O(n + sqrt(S) + d * k) where d is number of divisors
    /// - Space: O(n + S) for adjacency list and subtree sum counts
    pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }

        let s = nums.iter().sum::<i32>() as usize;
        if s == 0 {
            return (n - 1) as i32;
        }

        let mut counts = vec![0usize; s + 1];
        Self::dfs(&g, &nums, &mut counts, 0, n);

        let mut divisors = Vec::new();
        let mut i = 1;
        while i * i <= s {
            if s % i == 0 {
                divisors.push(i);
                if i != s / i {
                    divisors.push(s / i);
                }
            }
            i += 1;
        }
        divisors.sort_unstable_by(|a, b| b.cmp(a));

        for num_components in divisors {
            if num_components > n {
                continue;
            }
            let desired_sum = s / num_components;
            let multiple_count: usize = (desired_sum..=s)
                .step_by(desired_sum)
                .map(|m| counts[m])
                .sum();
            if multiple_count == num_components {
                return (num_components - 1) as i32;
            }
        }

        0
    }

    fn dfs(g: &[Vec<usize>], vals: &[i32], counts: &mut [usize], i: usize, parent: usize) -> i32 {
        let subtree_sum = g[i]
            .iter()
            .filter(|&&conn| conn != parent)
            .fold(vals[i], |sum, &conn| {
                sum + Self::dfs(g, vals, counts, conn, i)
            });
        counts[subtree_sum as usize] += 1;
        subtree_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![6, 2, 2, 2, 6];
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
        assert_eq!(Solution::component_value(nums, edges), 2);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::component_value(vec![2], vec![]), 0);
    }

    #[test]
    fn test_no_valid_partition() {
        let nums = vec![1, 2, 3];
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::component_value(nums, edges), 0);
    }

    #[test]
    fn test_all_same_values() {
        let nums = vec![5, 5, 5, 5];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::component_value(nums, edges), 3);
    }

    #[test]
    fn test_two_components() {
        let nums = vec![4, 6, 6, 4];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::component_value(nums, edges), 1);
    }

    #[test]
    fn test_indivisible_sum() {
        let nums = vec![50, 1, 1, 1, 1];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::component_value(nums, edges), 0);
    }
}
