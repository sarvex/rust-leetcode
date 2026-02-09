impl Solution {
    /// Maximize total points with at least k tasks using technique 1.
    ///
    /// # Intuition
    /// Start from "all technique 2" (sum of technique2). Switching task i to technique 1
    /// adds gain[i] = technique1[i] - technique2[i]. We must take at least k such gains;
    /// for the rest we take only positive gains.
    ///
    /// # Approach
    /// 1. Base = sum(technique2).
    /// 2. Gains = technique1[i] - technique2[i].
    /// 3. Use `select_nth_unstable_by` to partition the top-k gains in O(n) average time.
    /// 4. Sum the required top-k gains, then add positive gains from the remainder.
    ///
    /// # Complexity
    /// - Time: O(n) average
    /// - Space: O(n)
    pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;

        let base: i64 = technique2.iter().map(|&x| x as i64).sum();
        let mut gains: Vec<i32> = technique1
            .iter()
            .zip(technique2.iter())
            .map(|(a, b)| a - b)
            .collect();

        if k == 0 {
            let extra: i64 = gains.iter().filter(|&&g| g > 0).map(|&g| g as i64).sum();
            return base + extra;
        }

        gains.select_nth_unstable_by(k - 1, |a, b| b.cmp(a));

        let required: i64 = gains[..k].iter().map(|&g| g as i64).sum();
        let extra: i64 = gains[k..].iter().filter(|&&g| g > 0).map(|&g| g as i64).sum();

        base + required + extra
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::max_points(vec![5, 2, 10], vec![10, 3, 8], 2),
            22
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::max_points(vec![10, 20, 30], vec![5, 15, 25], 2),
            60
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::max_points(vec![1, 2, 3], vec![4, 5, 6], 0),
            15
        );
    }

    #[test]
    fn test_k_equals_n() {
        assert_eq!(
            Solution::max_points(vec![10, 20], vec![1, 2], 2),
            30
        );
    }

    #[test]
    fn test_single_task_k1() {
        assert_eq!(Solution::max_points(vec![10], vec![5], 1), 10);
    }

    #[test]
    fn test_single_task_k0() {
        assert_eq!(Solution::max_points(vec![10], vec![5], 0), 10);
    }
}
