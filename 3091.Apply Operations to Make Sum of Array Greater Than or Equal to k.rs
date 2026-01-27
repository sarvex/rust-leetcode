impl Solution {
    /// Minimum operations (increment 1 or duplicate any element) to reach sum >= k.
    ///
    /// # Intuition
    /// Start with a single element of value 1. Incrementing a times gives value (a+1).
    /// Then duplicating b times yields (b+1) copies, for a total of (a+1)*(b+1).
    /// Minimize a + b such that (a+1)*(b+1) >= k.
    ///
    /// # Approach
    /// 1. Enumerate all possible increment counts a from 0 to k-1.
    /// 2. For each a, compute the minimum duplicates b = ceil(k / (a+1)) - 1.
    /// 3. Track the minimum total operations a + b.
    ///
    /// # Complexity
    /// - Time: O(k)
    /// - Space: O(1)
    pub fn min_operations(k: i32) -> i32 {
        (0..k)
            .map(|a| {
                let x = a + 1;
                let b = (k + x - 1) / x - 1;
                a + b
            })
            .min()
            .unwrap_or(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_target() {
        assert_eq!(Solution::min_operations(1), 0);
    }

    #[test]
    fn target_eleven() {
        assert_eq!(Solution::min_operations(11), 5);
    }
}
