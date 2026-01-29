impl Solution {
    /// Computes the average of even numbers divisible by three (i.e., divisible by 6).
    ///
    /// # Intuition
    /// A number that is both even and divisible by 3 must be divisible by 6.
    /// Filter and average those elements.
    ///
    /// # Approach
    /// 1. Filter elements divisible by 6
    /// 2. Accumulate sum and count in a single fold
    /// 3. Return integer division of sum by count, or 0 if none found
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (sum, count) = nums
            .iter()
            .filter(|x| **x % 6 == 0)
            .fold((0, 0), |(s, n), &x| (s + x, n + 1));

        if count == 0 {
            0
        } else {
            sum / count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::average_value(vec![1, 3, 6, 10, 12, 15]), 9);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::average_value(vec![1, 2, 4, 7, 10]), 0);
    }

    #[test]
    fn test_all_divisible_by_six() {
        assert_eq!(Solution::average_value(vec![6, 12, 18]), 12);
    }

    #[test]
    fn test_single_valid() {
        assert_eq!(Solution::average_value(vec![1, 6, 7]), 6);
    }

    #[test]
    fn test_no_valid() {
        assert_eq!(Solution::average_value(vec![1, 3, 5]), 0);
    }
}
