impl Solution {
    /// Greedy outward expansion from the deficit position.
    ///
    /// # Intuition
    /// At most one index is negative. Expand outward from it, greedily collecting
    /// supply from the closest positions first (both clockwise and counter-clockwise).
    ///
    /// # Approach
    /// 1. If total sum < 0, impossible → return -1.
    /// 2. Find the minimum element (the sole negative); if none, return 0.
    /// 3. Walk outward at distances 1, 2, …, checking left and right neighbors.
    ///    Take as much as needed from each, accumulating cost = amount * distance.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_moves(balance: Vec<i32>) -> i64 {
        let total: i64 = balance.iter().map(|&b| b as i64).sum();
        if total < 0 {
            return -1;
        }

        let n = balance.len();
        let (idx, &min_val) = balance.iter().enumerate().min_by_key(|&(_, &v)| v).unwrap();
        if min_val >= 0 {
            return 0;
        }

        let mut need = (-min_val) as i64;
        let mut ans = 0_i64;

        for j in 1..n {
            let left = balance[(idx + n - j) % n] as i64;
            let take = left.min(need);
            need -= take;
            ans += take * j as i64;

            let right = balance[(idx + j) % n] as i64;
            let take = right.min(need);
            need -= take;
            ans += take * j as i64;

            if need == 0 {
                break;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_moves(vec![5, 1, -4]), 4);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_moves(vec![1, 2, -5, 2]), 6);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_moves(vec![-3, 2]), -1);
    }

    #[test]
    fn test_no_negative() {
        assert_eq!(Solution::min_moves(vec![1, 2, 3]), 0);
    }

    #[test]
    fn test_single_negative_zero_sum() {
        assert_eq!(Solution::min_moves(vec![2, -2]), 2);
    }
}
