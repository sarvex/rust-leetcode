pub struct Solution;

impl Solution {
    /// Checks for an increasing triplet subsequence using two running minimums.
    ///
    /// # Intuition
    /// Track the smallest and second-smallest values seen so far. If any value
    /// exceeds both, a triplet exists.
    ///
    /// # Approach
    /// 1. Initialize `first` and `second` to MAX.
    /// 2. For each number: update first if <= first, else update second if <= second,
    ///    else return true (triplet found).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first, mut second) = (i32::MAX, i32::MAX);
        for num in nums {
            if num <= first {
                first = num;
            } else if num <= second {
                second = num;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_triplet() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn no_triplet_decreasing() {
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn non_consecutive_triplet() {
        assert!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }

    #[test]
    fn exact_three_elements() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3]));
    }

    #[test]
    fn two_elements_only() {
        assert!(!Solution::increasing_triplet(vec![1, 2]));
    }

    #[test]
    fn all_same() {
        assert!(!Solution::increasing_triplet(vec![1, 1, 1, 1]));
    }

    #[test]
    fn empty_array() {
        assert!(!Solution::increasing_triplet(vec![]));
    }

    #[test]
    fn single_element() {
        assert!(!Solution::increasing_triplet(vec![1]));
    }

    #[test]
    fn triplet_with_gaps() {
        assert!(Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]));
    }

    #[test]
    fn negative_numbers() {
        assert!(Solution::increasing_triplet(vec![-5, -4, -3]));
    }

    #[test]
    fn mixed_signs() {
        assert!(Solution::increasing_triplet(vec![-2, 0, 2]));
    }
}
