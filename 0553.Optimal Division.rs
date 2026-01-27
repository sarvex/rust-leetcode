impl Solution {
    /// Finds the optimal division expression that maximizes the result.
    ///
    /// # Intuition
    /// To maximize a/b/c/d/..., make the denominator as small as possible.
    /// Grouping all elements after the first as (b/c/d/...) forces them into
    /// a single division chain that minimizes the denominator.
    ///
    /// # Approach
    /// 1. If only one number, return it directly.
    /// 2. If two numbers, return "a/b".
    /// 3. Otherwise return "a/(b/c/.../z)".
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn optimal_division(nums: Vec<i32>) -> String {
        match nums.len() {
            1 => nums[0].to_string(),
            2 => format!("{}/{}", nums[0], nums[1]),
            _ => {
                let inner: Vec<String> = nums[1..].iter().map(|n| n.to_string()).collect();
                format!("{}/({})", nums[0], inner.join("/"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three() {
        assert_eq!(
            Solution::optimal_division(vec![1000, 100, 10, 2]),
            "1000/(100/10/2)"
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(Solution::optimal_division(vec![2, 3]), "2/3");
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::optimal_division(vec![5]), "5");
    }
}
