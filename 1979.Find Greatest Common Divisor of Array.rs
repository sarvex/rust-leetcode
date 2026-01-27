impl Solution {
    /// Computes the GCD of the minimum and maximum array elements.
    ///
    /// # Intuition
    /// The problem reduces to finding GCD of the array's extremes.
    ///
    /// # Approach
    /// 1. Find min and max values.
    /// 2. Apply Euclidean GCD algorithm.
    ///
    /// # Complexity
    /// - Time: O(n + log(min(a, b)))
    /// - Space: O(1)
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
    }

    #[test]
    fn test_coprime() {
        assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::find_gcd(vec![4, 4, 4]), 4);
    }
}
