impl Solution {
    /// Maximum total capacity with at most two machines and total cost strictly below budget.
    ///
    /// # Intuition
    /// Sort by cost; for each machine i consider it as the "right" machine and pair with the best
    /// machine to its left that satisfies cost[i] + cost[j] < budget. Using a single prefix-max
    /// and a right-to-left pointer j avoids second-best bookkeeping.
    ///
    /// # Approach
    /// 1. Sort (cost, capacity) by cost; `(i32, i32)` is `Ord`, so `sort_unstable` is valid and fast.
    /// 2. Single pass: maintain prefix max capacity `p[i]`, and pointer `j` = largest index with
    ///    `cost[j] + cost[i] < budget`. For each i, pair with best in 0..=min(j, i-1) = `p[k]`;
    ///    since k < i we never pair i with itself. Also track single-machine best (cost < budget).
    /// 3. Return the maximum capacity seen.
    ///
    /// # Complexity
    /// - Time: O(n log n) from sort; single pass with two-pointer is O(n).
    /// - Space: O(n)
    pub fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 {
        let n = costs.len();
        let mut t: Vec<(i32, i32)> = costs.into_iter().zip(capacity.into_iter()).collect();
        t.sort_unstable();

        let mut p = vec![0i32; n];
        let mut ans = 0i32;
        let mut j = n as i32 - 1;

        for (i, &(c, v)) in t.iter().enumerate() {
            let prev = if i == 0 { 0 } else { p[i - 1] };
            p[i] = prev.max(v);
            if c < budget {
                ans = ans.max(v);
            }
            while j >= 0 && t[j as usize].0 + c >= budget {
                j -= 1;
            }
            let k = j.min(i as i32 - 1);
            if k >= 0 {
                ans = ans.max(v + p[k as usize]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::max_capacity(vec![4, 8, 5, 3], vec![1, 5, 2, 7], 8),
            8
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::max_capacity(vec![3, 5, 7, 4], vec![2, 4, 3, 6], 7),
            6
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::max_capacity(vec![2, 2, 2], vec![3, 5, 4], 5),
            9
        );
    }

    #[test]
    fn test_single_machine_best() {
        assert_eq!(Solution::max_capacity(vec![1, 10], vec![5, 100], 5), 5);
    }

    #[test]
    fn test_two_machines_same_cost() {
        assert_eq!(
            Solution::max_capacity(vec![3, 3], vec![2, 4], 7),
            6
        );
    }

    #[test]
    fn test_none_affordable() {
        assert_eq!(Solution::max_capacity(vec![10, 10], vec![1, 2], 5), 0);
    }

    #[test]
    fn test_one_element() {
        assert_eq!(Solution::max_capacity(vec![2], vec![7], 5), 7);
    }
}
