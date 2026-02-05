impl Solution {
    /// Finds the length of the longest increasing subsequence using binary search.
    ///
    /// # Intuition
    /// Maintain a "tails" array where tails[i] is the smallest ending element
    /// of all increasing subsequences of length i+1. Binary search to find
    /// where each element fits.
    ///
    /// # Approach
    /// 1. For each number, binary search for the first tail >= num.
    /// 2. If found, replace that tail with num (smaller ending extends more).
    /// 3. If not found, append num (extends the longest subsequence).
    /// 4. The length of tails is the LIS length.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(Vec::with_capacity(nums.len()), |mut tails, &num| {
                match tails.binary_search(&num) {
                    Ok(_) => {}
                    Err(pos) if pos == tails.len() => tails.push(num),
                    Err(pos) => tails[pos] = num,
                }
                tails
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        // LIS: [2, 3, 7, 101] or [2, 5, 7, 101] or [2, 3, 7, 18] etc.
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn mixed_sequence() {
        // LIS: [0, 1, 2, 3] length 4
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn all_same() {
        // No increasing subsequence longer than 1
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7]), 1);
    }

    #[test]
    fn strictly_increasing() {
        // Entire array is the LIS
        assert_eq!(Solution::length_of_lis(vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn strictly_decreasing() {
        // LIS is any single element
        assert_eq!(Solution::length_of_lis(vec![5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::length_of_lis(vec![1]), 1);
    }

    #[test]
    fn two_elements_increasing() {
        assert_eq!(Solution::length_of_lis(vec![1, 2]), 2);
    }

    #[test]
    fn two_elements_decreasing() {
        assert_eq!(Solution::length_of_lis(vec![2, 1]), 1);
    }

    #[test]
    fn negative_numbers() {
        // LIS: [-5, -2, 0, 3] length 4
        assert_eq!(Solution::length_of_lis(vec![-5, -2, -1, 0, -3, 3]), 5);
    }

    #[test]
    fn alternating() {
        // LIS: [1, 2, 3] length 3
        assert_eq!(Solution::length_of_lis(vec![1, 3, 2, 4, 3, 5]), 4);
    }
}
