impl Solution {
    /// Computes the absolute difference between element sum and digit sum.
    ///
    /// # Intuition
    /// Element sum is always >= digit sum for positive integers, since each element
    /// contributes at least as much as its digits. The difference is always non-negative.
    ///
    /// # Approach
    /// Compute element sum and digit sum in a single pass, return the difference.
    ///
    /// # Complexity
    /// - Time: O(n × d) where d is the average number of digits per element
    /// - Space: O(1)
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let (elem_sum, digit_sum) = nums.iter().fold((0, 0), |(es, ds), &v| {
            let mut d = 0;
            let mut num = v;
            while num > 0 {
                d += num % 10;
                num /= 10;
            }
            (es + v, ds + d)
        });

        (elem_sum - digit_sum).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
    }

    #[test]
    fn test_single_digits() {
        assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(Solution::difference_of_sum(vec![100]), 99);
    }
}
