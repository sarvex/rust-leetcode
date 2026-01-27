impl Solution {
    /// Finds the majority element using Boyer-Moore Voting Algorithm.
    ///
    /// # Intuition
    /// The majority element appears more than n/2 times, so its count will always
    /// be positive after canceling out with other elements.
    ///
    /// # Approach
    /// 1. Initialize a candidate and count.
    /// 2. When count reaches 0, adopt the current element as the new candidate.
    /// 3. Increment count for matches, decrement for mismatches.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;
        for &num in &nums {
            if count == 0 {
                candidate = num;
                count = 1;
            } else if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_length_array() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn even_length_array() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::majority_element(vec![1]), 1);
    }
}
