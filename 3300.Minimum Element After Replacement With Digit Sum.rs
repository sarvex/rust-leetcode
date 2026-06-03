impl Solution {
    /// Find minimum element after replacing each number with its digit sum.
    ///
    /// # Intuition
    /// Replace each number with the sum of its digits, then find the minimum.
    /// We can compute both in a single iterator chain.
    ///
    /// # Approach
    /// For each number, repeatedly extract digits via modulo and division,
    /// accumulating their sum. Then take the minimum across all transformed values.
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is the number of digits (at most 4 for nums[i] <= 10^4)
    /// - Space: O(1)
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(|&n| {
                let (mut num, mut sum) = (n, 0);
                while num > 0 {
                    sum += num % 10;
                    num /= 10;
                }
                sum
            })
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_element(vec![10, 12, 13, 14]), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_element(vec![1, 2, 3, 4]), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_element(vec![999, 19, 199]), 10);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_element(vec![9999]), 36);
    }

    #[test]
    fn test_max_value() {
        // 10000 -> 1+0+0+0+0 = 1
        assert_eq!(Solution::min_element(vec![10000]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::min_element(vec![11, 11, 11]), 2);
    }
}
