impl Solution {
    /// Linear scan to find special value x.
    ///
    /// # Intuition
    /// A value x is special if exactly x elements are ≥ x. Testing each
    /// candidate from 0 to n and counting qualifying elements finds the answer.
    ///
    /// # Approach
    /// 1. For each x in 0..=n, count elements ≥ x
    /// 2. Return x if count equals x, otherwise -1
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(1)
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        (0..=n)
            .find(|&x| nums.iter().filter(|v| **v >= x).count() as i32 == x)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_exists() {
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
    }

    #[test]
    fn no_special() {
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
    }

    #[test]
    fn special_zero_threshold() {
        assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    }
}
