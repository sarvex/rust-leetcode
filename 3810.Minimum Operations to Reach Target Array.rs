impl Solution {
    /// Minimum operations = number of distinct values at indices where nums[i] != target[i].
    ///
    /// # Intuition
    /// Each operation picks a value x and updates every maximal segment of x to the corresponding
    /// target slice. So one operation fixes all positions that currently have value x. Wrong
    /// positions only shrink; fixing x never changes positions that have another value. Thus we
    /// need one operation per distinct "wrong" value, and that many operations suffice.
    ///
    /// # Approach
    /// Single pass: for each index where nums[i] != target[i], mark seen[nums[i]] = true using a
    /// fixed-size buffer (values in [1, 10^5]). Count true entries. No hashing, no heap per value.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed 10^5+1 bytes.
    pub fn min_operations(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        const MAX_NUM: usize = 100001;
        let mut set = [0u8; MAX_NUM];
        let mut result = 0;
        nums.iter()
            .zip(target.iter())
            .filter(|(n, t)| n != t)
            .for_each(|(n, _)| {
                let idx = *n as usize;
                if set[idx] == 0 {
                    set[idx] = 1;
                    result += 1;
                }
            });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3], vec![2, 1, 3]), 2);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_operations(vec![4, 1, 4], vec![5, 1, 4]), 1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_operations(vec![7, 3, 7], vec![5, 5, 9]), 2);
    }

    #[test]
    fn test_already_equal() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3], vec![1, 2, 3]), 0);
    }

    #[test]
    fn test_single_wrong() {
        assert_eq!(Solution::min_operations(vec![1], vec![2]), 1);
    }

    #[test]
    fn test_same_value_many_wrong() {
        assert_eq!(Solution::min_operations(vec![4, 4, 4], vec![1, 2, 3]), 1);
    }
}
