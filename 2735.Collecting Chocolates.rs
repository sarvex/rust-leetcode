impl Solution {
    /// Minimum cost to collect all chocolate types with rotation operations.
    ///
    /// # Intuition
    /// After `j` rotations (costing `x * j`), type `i` can be bought at the
    /// minimum price among positions `i, i-1, ..., i-j` (mod n). Precompute
    /// the running minimum for each type across all rotation counts.
    ///
    /// # Approach
    /// 1. Build matrix `f[i][j]` = min cost for type `i` after `j` rotations.
    /// 2. For each rotation count `j`, sum `f[i][j]` over all types plus `x * j`.
    /// 3. Return the global minimum total cost.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²)
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut f = vec![vec![0i32; n]; n];
        for i in 0..n {
            f[i][0] = nums[i];
            for j in 1..n {
                f[i][j] = f[i][j - 1].min(nums[(i + n - j) % n]);
            }
        }
        (0..n)
            .map(|j| {
                let rotation_cost = (x as i64) * (j as i64);
                let buy_cost: i64 = (0..n).map(|i| f[i][j] as i64).sum();
                rotation_cost + buy_cost
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rotation_cheapest() {
        assert_eq!(Solution::min_cost(vec![20, 1, 15], 5), 13);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::min_cost(vec![5], 10), 5);
    }

    #[test]
    fn rotation_beneficial() {
        assert_eq!(Solution::min_cost(vec![1, 2, 3], 4), 6);
    }
}
