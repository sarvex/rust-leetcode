impl Solution {
    /// Rotate only non-negative elements left by k positions; negatives stay fixed.
    ///
    /// # Intuition
    /// Collect non-negative elements, then stream them back with an offset
    /// to simulate left rotation without extra passes.
    ///
    /// # Approach
    /// 1. Collect non-negative values in order.
    /// 2. Compute the start offset `k % m` (m = count of non-negatives).
    /// 3. Walk the array again and write values from the rotated stream
    ///    into non-negative positions.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(m) for extracted non-negative values
    pub fn rotate_elements(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let non_neg: Vec<_> = nums.iter().filter(|&&x| x >= 0).copied().collect();

        let m = non_neg.len();
        if m < 2 {
            return nums;
        }
        let mut read = (k as usize) % m;
        for value in nums.iter_mut() {
            if *value >= 0 {
                *value = non_neg[read];
                read += 1;
                if read == m {
                    read = 0;
                }
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::rotate_elements(vec![1, -2, 3, -4], 3),
            vec![3, -2, 1, -4]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::rotate_elements(vec![-3, -2, 7], 1),
            vec![-3, -2, 7]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::rotate_elements(vec![5, 4, -9, 6], 2),
            vec![6, 5, -9, 4]
        );
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(
            Solution::rotate_elements(vec![-1, -2, -3], 5),
            vec![-1, -2, -3]
        );
    }

    #[test]
    fn test_single_non_negative() {
        assert_eq!(Solution::rotate_elements(vec![1], 0), vec![1]);
        assert_eq!(Solution::rotate_elements(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_k_larger_than_count() {
        assert_eq!(
            Solution::rotate_elements(vec![1, 2, -1, 3], 7),
            Solution::rotate_elements(vec![1, 2, -1, 3], 1)
        );
    }
}
