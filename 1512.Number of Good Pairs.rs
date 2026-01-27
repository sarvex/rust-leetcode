impl Solution {
    /// Counting array approach for good pairs.
    ///
    /// # Intuition
    /// A good pair (i, j) with i < j has `nums[i] == nums[j]`. For each
    /// element, the number of good pairs it forms equals the count of
    /// previous occurrences of the same value.
    ///
    /// # Approach
    /// 1. Maintain a frequency array (values 1..100)
    /// 2. For each element, add its current count to the answer, then increment
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed-size count array
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = [0i32; 101];
        nums.iter().fold(0, |pairs, &x| {
            let prev = count[x as usize];
            count[x as usize] += 1;
            pairs + prev
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_pairs() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }

    #[test]
    fn no_pairs() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
