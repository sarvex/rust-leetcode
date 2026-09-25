impl Solution {
    /// Smallest index whose value has a digit sum equal to that index.
    ///
    /// # Intuition
    /// Values are at most `1000`, so a digit sum is at most `1 + 9 + 9 + 9 = 28`.
    /// Only indices in `0..=28` can match. The first hit while scanning left to
    /// right is the smallest such index.
    ///
    /// # Approach
    /// For each index `i`, peel digits off `nums[i]` and add them. Return `i` as
    /// soon as that sum equals `i`. If the scan ends with no match, return `-1`.
    ///
    /// # Complexity
    /// - Time: O(n · d), where `d ≤ 4` is the number of digits
    /// - Space: O(1)
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .find(|(i, num)| digit_sum(**num) == *i as i32)
            .map_or(-1, |(i, _)| i as i32)
    }
}

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_match_at_last_index() {
        assert_eq!(Solution::smallest_index(vec![1, 3, 2]), 2);
    }

    #[test]
    fn example_smallest_of_two_matches() {
        assert_eq!(Solution::smallest_index(vec![1, 10, 11]), 1);
    }

    #[test]
    fn example_no_match() {
        assert_eq!(Solution::smallest_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn zero_at_index_zero() {
        assert_eq!(Solution::smallest_index(vec![0]), 0);
    }

    #[test]
    fn single_nonzero() {
        assert_eq!(Solution::smallest_index(vec![1]), -1);
    }

    #[test]
    fn index_zero_beats_later_match() {
        assert_eq!(Solution::smallest_index(vec![0, 10, 11]), 0);
    }

    #[test]
    fn max_value_digit_sum() {
        // 1000 → 1, matches index 1
        assert_eq!(Solution::smallest_index(vec![5, 1000]), 1);
        // 999 → 27. Leading zeros would match index 0, so index 0 stays nonzero.
        let mut nums = vec![0; 28];
        nums[0] = 1;
        nums[27] = 999;
        assert_eq!(Solution::smallest_index(nums), 27);
    }
}
